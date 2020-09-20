pub mod tac;

use crate::frontend::parser::ast::*;
use crate::middleend::tacgen::tac::*;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct TacGen {
    reg: u32,
    label: u32,
    stack_offset: u32,

    ctx: Context,
}

#[derive(Debug)]
struct Context(Vec<HashMap<String, u32>>);

impl Deref for Context {
    type Target = Vec<HashMap<String, u32>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Context {
    fn new() -> Self {
        let mut ctx = Self(Vec::new());
        ctx.push_ctx();
        ctx
    }

    fn add_variable(&mut self, name: String, offset: u32) {
        self.last_mut().unwrap().insert(name, offset);
    }

    fn find_variable(&self, name: &String) -> u32 {
        for ctx in self.iter().rev() {
            if ctx.contains_key(name) {
                return *ctx.get(name).unwrap();
            }
        }
        unreachable!();
    }

    fn push_ctx(&mut self) {
        self.push(HashMap::new());
    }

    fn pop_ctx(&mut self) {
        self.pop();
    }

    fn clear_ctx(&mut self) {
        self.clear();
        self.push_ctx();
    }
}

pub fn generate(program: Program) -> Result<TacProgram, String> {
    let mut generator = TacGen::new();
    generator.generate(program)
}

impl TacGen {
    fn new() -> Self {
        Self {
            reg: 0,
            label: 0,
            stack_offset: 0,
            ctx: Context::new(),
        }
    }

    fn generate(&mut self, program: Program) -> Result<TacProgram, String> {
        let mut tac_program = TacProgram::new();
        for function in program.functions {
            tac_program.functions.push(self.gen_function(function)?);
        }
        Ok(tac_program)
    }

    fn gen_function(&mut self, func: Function) -> Result<TacFunction, String> {
        self.ctx.clear_ctx();
        self.stack_offset = 0;
        let mut tac_func = TacFunction::new(func.name.to_owned());
        self.gen_statement(func.body, &mut tac_func)?;
        Ok(tac_func)
    }

    fn gen_statement(&mut self, stmt: AstStatement, func: &mut TacFunction) -> Result<(), String> {
        match stmt {
            AstStatement::Block { stmts } => {
                self.ctx.push_ctx();
                for stmt in stmts {
                    self.gen_statement(stmt, func)?;
                }
                self.ctx.pop_ctx();
            }
            AstStatement::Declare {
                name,
                typ: _,
                value,
            } => {
                let offset = self.alloc_stack();
                self.ctx.add_variable(name.to_owned(), offset);
                self.gen_assign(offset, *value, func)?;
            }
            AstStatement::Assign { name, value } => {
                let offset = self.ctx.find_variable(&name);
                self.gen_assign(offset, *value, func)?;
            }
            AstStatement::Return { value } => {
                let src = self.gen_expression(*value, func)?;
                func.body.push(Tac::Ret { src });
            }
            AstStatement::If { cond, then, els } => {
                let label1 = self.next_label();

                let cond = self.gen_expression(*cond, func)?;
                func.body.push(Tac::JumpIfNot {
                    label_index: label1,
                    cond,
                });
                self.gen_statement(*then, func)?;

                if let Some(els) = els {
                    let label2 = self.next_label();

                    func.body.push(Tac::Jump {
                        label_index: label2,
                    });
                    func.body.push(Tac::Label { index: label1 });
                    self.gen_statement(*els, func)?;
                    func.body.push(Tac::Label { index: label2 });
                } else {
                    func.body.push(Tac::Label { index: label1 });
                }
            }
        }
        Ok(())
    }

    fn gen_expression(
        &mut self,
        expr: AstExpression,
        func: &mut TacFunction,
    ) -> Result<Operand, String> {
        match expr {
            AstExpression::Integer { value } => {
                let dst = Operand::Reg(self.next_reg());
                func.body.push(Tac::Move {
                    dst: dst.clone(),
                    src: Operand::Const(value),
                });
                Ok(dst)
            }
            AstExpression::Bool { value } => {
                let dst = Operand::Reg(self.next_reg());
                func.body.push(Tac::Move {
                    dst: dst.clone(),
                    src: Operand::Const(value as i32),
                });
                Ok(dst)
            }
            AstExpression::Ident { name } => {
                let offset = self.ctx.find_variable(&name);
                Ok(Operand::Variable(offset))
            }
            AstExpression::UnaryOp { op, expr } => {
                let src = self.gen_expression(*expr, func)?;
                func.body.push(Tac::UnOp {
                    op,
                    src: src.clone(),
                });
                Ok(src)
            }
            AstExpression::BinaryOp { op, lhs, rhs } => {
                let lhs = self.gen_expression(*lhs, func)?;
                let rhs = self.gen_expression(*rhs, func)?;
                let dst = Operand::Reg(self.next_reg());
                func.body.push(Tac::BinOp {
                    op,
                    dst: dst.clone(),
                    lhs,
                    rhs,
                });
                Ok(dst)
            }
        }
    }

    fn gen_assign(
        &mut self,
        offset: u32,
        src: AstExpression,
        func: &mut TacFunction,
    ) -> Result<(), String> {
        let dst = Operand::Variable(offset);
        let src = self.gen_expression(src, func)?;
        let src_reg = Operand::Reg(self.next_reg());
        func.body.push(Tac::Move {
            dst: src_reg.clone(),
            src,
        });
        func.body.push(Tac::Move { dst, src: src_reg });
        Ok(())
    }

    fn next_reg(&mut self) -> RegisterInfo {
        let cur_reg = self.reg;
        self.reg += 1;
        RegisterInfo {
            virtual_index: cur_reg,
            physical_index: None,
        }
    }

    fn next_label(&mut self) -> u32 {
        let cur_label = self.label;
        self.label += 1;
        cur_label
    }

    fn alloc_stack(&mut self) -> u32 {
        self.stack_offset += 4;
        self.stack_offset
    }
}
