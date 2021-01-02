pub mod ir;

use crate::{
    common::error::{Error, ErrorKind},
    frontend::parser::ast::*,
    middleend::irgen::ir::*,
};
use std::collections::HashMap;

#[derive(Debug)]
struct IRGen {
    cur_funcname: String,
    reg: u32,
    label: u32,

    stack_offset_local: i32,

    ctx: Context,
}

#[derive(Debug)]
struct Context(Vec<HashMap<String, MemoryAddr>>);

impl Context {
    fn new() -> Self {
        let mut ctx = Self(Vec::new());
        ctx.push();
        ctx
    }

    fn add_variable(&mut self, name: String, addr: MemoryAddr) {
        self.0.last_mut().unwrap().insert(name, addr);
    }

    fn find_variable(&self, name: &str) -> MemoryAddr {
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

pub fn generate(program: Program) -> Result<IRProgram, Error> {
    let mut generator = IRGen::new();
    Ok(generator.generate(program)?)
}

impl IRGen {
    fn new() -> Self {
        Self {
            cur_funcname: "".into(),
            reg: 0,
            label: 0,
            stack_offset_local: 0,
            ctx: Context::new(),
        }
    }

    fn generate(&mut self, program: Program) -> Result<IRProgram, Error> {
        let mut ir_program = IRProgram::default();
        for function in program.functions {
            if let Some(func) = self.gen_function(function)? {
                ir_program.functions.push(func);
            }
        }
        Ok(ir_program)
    }

    fn gen_function(&mut self, func: Function) -> Result<Option<IRFunction>, Error> {
        if func.body.is_none() {
            return Ok(None);
        }

        self.init();
        self.cur_funcname = func.name.clone();
        let mut ir_func = IRFunction::new(func.name.to_owned());
        ir_func.new_block(format!(".L.{}.entry", func.name));
        for (index, param) in func.params.iter().enumerate() {
            let addr = self.alloc_stack_local();
            self.ctx.add_variable(param.name.to_owned(), addr);
            ir_func.push(IR::StoreArg {
                dst: addr,
                src: index,
            });
            ir_func.params.push(index as u32);
        }
        self.gen_statement(func.body.unwrap(), &mut ir_func)?;
        Ok(Some(ir_func))
    }

    fn gen_statement(&mut self, stmt: Statement, func: &mut IRFunction) -> Result<(), Error> {
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
                let addr = self.alloc_stack_local();
                self.ctx.add_variable(name, addr);

                let dst = self.next_reg();
                func.push(IR::Addr { dst, src: addr });

                // init by 0 if value is None
                let src = value.map(|value| *value).unwrap_or(Expression::new(
                    ExpressionKind::Integer { value: 0 },
                    stmt.pos,
                ));

                self.gen_assign(dst, src, func)?;
            }
            StatementKind::Assign { dst, value } => {
                let dst = self.gen_lvalue(*dst, func)?;
                self.gen_assign(dst, *value, func)?;
            }
            StatementKind::Return { value } => {
                let src = match value {
                    Some(value) => Some(self.gen_expression(*value, func)?),
                    None => None,
                };
                func.push(IR::Ret { src });
            }
            StatementKind::If { cond, then, els } => {
                let label1 = self.next_label();

                let cond = self.gen_expression(*cond, func)?;
                func.push(IR::JumpIfNot {
                    label: label1.to_owned(),
                    cond,
                });
                self.gen_statement(*then, func)?;

                if let Some(els) = els {
                    let label2 = self.next_label();

                    func.push(IR::Jump {
                        label: label2.to_owned(),
                    });
                    func.new_block(label1);
                    self.gen_statement(*els, func)?;
                    func.new_block(label2);
                } else {
                    func.new_block(label1);
                }
            }
            StatementKind::While { cond, body } => {
                let label1 = self.next_label();
                let label2 = self.next_label();

                // condition
                func.new_block(label1.to_owned());
                let cond = self.gen_expression(*cond, func)?;
                func.push(IR::JumpIfNot {
                    label: label2.to_owned(),
                    cond,
                });

                // body
                self.gen_statement(*body, func)?;
                func.push(IR::Jump { label: label1 });

                func.new_block(label2);
            }
            StatementKind::Call { name, args } => self.gen_call(None, name, args, func)?,
        }
        Ok(())
    }

    fn gen_expression(
        &mut self,
        expr: Expression,
        func: &mut IRFunction,
    ) -> Result<Operand, Error> {
        match expr.kind {
            ExpressionKind::Integer { value } => {
                let dst = self.next_reg();
                func.push(IR::Move {
                    dst,
                    src: Operand::Const(value),
                });
                Ok(dst)
            }
            ExpressionKind::Bool { value } => {
                let dst = self.next_reg();
                func.push(IR::Move {
                    dst,
                    src: Operand::Const(value as i32),
                });
                Ok(dst)
            }
            ExpressionKind::Ident { .. } => {
                let dst = self.next_reg();
                let src = self.gen_lvalue(expr, func)?;
                func.push(IR::Load { dst, src });
                Ok(dst)
            }
            ExpressionKind::UnaryOp { op, expr } => {
                let src = self.gen_expression(*expr, func)?;
                func.push(IR::UnOp { op, src });
                Ok(src)
            }
            ExpressionKind::BinaryOp { op, lhs, rhs } => {
                let lhs = self.gen_expression(*lhs, func)?;
                let rhs = self.gen_expression(*rhs, func)?;
                let dst = self.next_reg();
                func.push(IR::BinOp { op, dst, lhs, rhs });
                Ok(dst)
            }
            ExpressionKind::Call { name, args } => {
                let dst = self.next_reg();
                self.gen_call(Some(dst), name, args, func)?;
                Ok(dst)
            }
            ExpressionKind::Index { lhs, index } => {
                let dst = self.next_reg();
                // let lhs_addr = self.gen_lvalue(*lhs)?;
                // // let offset = self

                // func.push(IR::Load { dst, src: lhs_addr });
                Ok(dst)
            }
        }
    }

    fn gen_lvalue(&mut self, expr: Expression, func: &mut IRFunction) -> Result<Operand, Error> {
        match expr.kind {
            ExpressionKind::Ident { name } => {
                let reg = self.next_reg();
                func.push(IR::Addr {
                    dst: reg,
                    src: self.ctx.find_variable(&name),
                });
                Ok(reg)
            }
            _ => Err(Error::new(expr.pos, ErrorKind::LvalueRequired)),
        }
    }

    fn gen_assign(
        &mut self,
        dst: Operand,
        src: Expression,
        func: &mut IRFunction,
    ) -> Result<(), Error> {
        let src = self.gen_expression(src, func)?;
        func.push(IR::Store { dst, src });
        Ok(())
    }

    fn gen_call(
        &mut self,
        dst: Option<Operand>,
        name: String,
        args: Vec<Expression>,
        func: &mut IRFunction,
    ) -> Result<(), Error> {
        let mut arg_operands = Vec::new();
        for arg in args {
            arg_operands.push(self.gen_expression(arg, func)?);
        }
        func.push(IR::Call {
            dst,
            name,
            args: arg_operands,
        });
        Ok(())
    }

    fn init(&mut self) {
        self.reg = 0;
        self.label = 0;
        self.stack_offset_local = 0;
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

    fn next_label(&mut self) -> String {
        let cur_label = self.label;
        self.label += 1;
        format!(".L.{}.{}", self.cur_funcname, cur_label)
    }

    fn alloc_stack_local(&mut self) -> MemoryAddr {
        self.stack_offset_local -= 8;
        MemoryAddr {
            base: Register::Rbp,
            offset: self.stack_offset_local,
        }
    }
}

impl IRFunction {
    fn push(&mut self, ir: IR) {
        let last_block = self.blocks.last_mut().unwrap();
        last_block.irs.push(ir);
    }

    fn new_block(&mut self, name: String) {
        self.blocks.push(IRBlock::new(name));
    }
}
