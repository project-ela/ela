pub mod tac;

use crate::{common::error::Error, frontend::parser::ast::*, middleend::tacgen::tac::*};
use std::collections::HashMap;

#[derive(Debug)]
struct TacGen {
    reg: u32,
    label: u32,
    stack_offset: u32,

    ctx: Context,
}

#[derive(Debug)]
struct Context(Vec<HashMap<String, Operand>>);

impl Context {
    fn new() -> Self {
        let mut ctx = Self(Vec::new());
        ctx.push();
        ctx
    }

    fn add_variable(&mut self, name: String, operand: Operand) {
        self.0.last_mut().unwrap().insert(name, operand);
    }

    fn find_variable(&self, name: &str) -> Operand {
        for ctx in self.0.iter().rev() {
            if ctx.contains_key(name) {
                return *ctx.get(name).unwrap();
            }
        }
        unreachable!();
    }

    fn push(&mut self) {
        self.0.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.0.pop();
    }

    fn clear(&mut self) {
        self.0.clear();
        self.push();
    }
}

pub fn generate(program: Program) -> Result<TacProgram, Error> {
    let mut generator = TacGen::new();
    Ok(generator.generate(program)?)
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

    fn generate(&mut self, program: Program) -> Result<TacProgram, Error> {
        let mut tac_program = TacProgram::default();
        for function in program.functions {
            tac_program.functions.push(self.gen_function(function)?);
        }
        Ok(tac_program)
    }

    fn gen_function(&mut self, func: Function) -> Result<TacFunction, Error> {
        self.init();
        let mut tac_func = TacFunction::new(func.name.to_owned());
        self.gen_statement(func.body, &mut tac_func)?;
        Ok(tac_func)
    }

    fn gen_statement(&mut self, stmt: Statement, func: &mut TacFunction) -> Result<(), Error> {
        match stmt.kind {
            StatementKind::Block { stmts } => {
                self.ctx.push();
                for stmt in stmts {
                    self.gen_statement(stmt, func)?;
                }
                self.ctx.pop();
            }
            StatementKind::Var {
                name,
                typ: _,
                value,
            }
            | StatementKind::Val {
                name,
                typ: _,
                value,
            } => {
                let operand = self.alloc_stack();
                self.ctx.add_variable(name, operand.clone());
                self.gen_assign(operand, *value, func)?;
            }
            StatementKind::Assign { name, value } => {
                let operand = self.ctx.find_variable(&name);
                self.gen_assign(operand, *value, func)?;
            }
            StatementKind::Return { value } => {
                let src = match value {
                    Some(value) => Some(self.gen_expression(*value, func)?),
                    None => None,
                };
                func.body.push(Tac::Ret { src });
            }
            StatementKind::If { cond, then, els } => {
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
            StatementKind::While { cond, body } => {
                let label1 = self.next_label();
                let label2 = self.next_label();

                // condition
                func.body.push(Tac::Label { index: label1 });
                let cond = self.gen_expression(*cond, func)?;
                func.body.push(Tac::JumpIfNot {
                    label_index: label2,
                    cond,
                });

                // body
                self.gen_statement(*body, func)?;
                func.body.push(Tac::Jump {
                    label_index: label1,
                });

                func.body.push(Tac::Label { index: label2 });
            }
            StatementKind::Call { name } => func.body.push(Tac::Call { dst: None, name }),
        }
        Ok(())
    }

    fn gen_expression(
        &mut self,
        expr: Expression,
        func: &mut TacFunction,
    ) -> Result<Operand, Error> {
        match expr.kind {
            ExpressionKind::Integer { value } => {
                let dst = self.next_reg();
                func.body.push(Tac::Move {
                    dst: dst.clone(),
                    src: Operand::Const(value),
                });
                Ok(dst)
            }
            ExpressionKind::Bool { value } => {
                let dst = self.next_reg();
                func.body.push(Tac::Move {
                    dst: dst.clone(),
                    src: Operand::Const(value as i32),
                });
                Ok(dst)
            }
            ExpressionKind::Ident { name } => {
                let operand = self.ctx.find_variable(&name);
                let dst = self.next_reg();
                func.body.push(Tac::Move {
                    dst: dst.clone(),
                    src: operand,
                });
                Ok(dst)
            }
            ExpressionKind::UnaryOp { op, expr } => {
                let src = self.gen_expression(*expr, func)?;
                func.body.push(Tac::UnOp {
                    op,
                    src: src.clone(),
                });
                Ok(src)
            }
            ExpressionKind::BinaryOp { op, lhs, rhs } => {
                let lhs = self.gen_expression(*lhs, func)?;
                let rhs = self.gen_expression(*rhs, func)?;
                let dst = self.next_reg();
                func.body.push(Tac::BinOp {
                    op,
                    dst: dst.clone(),
                    lhs,
                    rhs,
                });
                Ok(dst)
            }
            ExpressionKind::Call { name } => {
                let dst = self.next_reg();
                func.body.push(Tac::Call {
                    dst: Some(dst.clone()),
                    name,
                });
                Ok(dst)
            }
        }
    }

    fn gen_assign(
        &mut self,
        dst: Operand,
        src: Expression,
        func: &mut TacFunction,
    ) -> Result<(), Error> {
        let src = self.gen_expression(src, func)?;
        let src_reg = self.next_reg();
        func.body.push(Tac::Move {
            dst: src_reg.clone(),
            src,
        });
        func.body.push(Tac::Move { dst, src: src_reg });
        Ok(())
    }

    fn init(&mut self) {
        self.reg = 0;
        self.label = 0;
        self.stack_offset = 0;
        self.ctx.clear();
    }

    fn next_reg(&mut self) -> Operand {
        let cur_reg = self.reg;
        self.reg += 1;
        Operand::Reg(RegisterInfo {
            virtual_index: cur_reg,
            physical_index: None,
        })
    }

    fn next_label(&mut self) -> u32 {
        let cur_label = self.label;
        self.label += 1;
        cur_label
    }

    fn alloc_stack(&mut self) -> Operand {
        self.stack_offset += 4;
        Operand::Variable(self.stack_offset)
    }
}
