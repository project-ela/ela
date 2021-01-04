pub mod ir;

use crate::{
    common::{
        error::Error,
        operator::{BinaryOperator, UnaryOperator},
        types::Type,
    },
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
struct Context(Vec<HashMap<String, Variable>>);

#[derive(Debug, Clone)]
struct Variable(MemoryAddr, Type);

impl Context {
    fn new() -> Self {
        let mut ctx = Self(Vec::new());
        ctx.push();
        ctx
    }

    fn add_variable(&mut self, name: String, addr: MemoryAddr, typ: Type) {
        self.0.last_mut().unwrap().insert(name, Variable(addr, typ));
    }

    fn find_variable(&self, name: &str) -> Variable {
        for ctx in self.0.iter().rev() {
            if ctx.contains_key(name) {
                return ctx.get(name).cloned().unwrap();
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
            let addr = self.alloc_stack_local(&param.typ);
            self.ctx
                .add_variable(param.name.to_owned(), addr, param.typ.clone());
            ir_func.push(IR::StoreArg {
                dst: addr,
                src: index,
            });
            ir_func.params.push(index as u32);
        }
        self.gen_statement(func.body.unwrap(), &mut ir_func)?;
        ir_func.stack_offset = (-self.stack_offset_local) as u32;
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
            StatementKind::Var { name, typ, value } | StatementKind::Val { name, typ, value } => {
                let addr = self.alloc_stack_local(&typ);
                self.ctx.add_variable(name, addr, typ.clone());

                match value {
                    Some(value) => {
                        let dst = self.next_reg();
                        func.push(IR::Addr { dst, src: addr });
                        self.gen_assign(dst, *value, func)?;
                    }
                    None => match typ {
                        Type::Array { elm_type, len } => {
                            for i in 0..len {
                                let dst = self.next_reg();
                                func.push(IR::Addr { dst, src: addr });
                                let offset = i * elm_type.size();
                                func.push(IR::BinOp {
                                    op: BinaryOperator::Add,
                                    dst,
                                    lhs: dst,
                                    rhs: Operand::Const(offset as i32),
                                });

                                let zero = Expression::new(
                                    ExpressionKind::Integer { value: 0 },
                                    stmt.pos.clone(),
                                );
                                self.gen_assign(dst, zero, func)?;
                            }
                        }
                        _ => {
                            let dst = self.next_reg();
                            func.push(IR::Addr { dst, src: addr });
                            let zero =
                                Expression::new(ExpressionKind::Integer { value: 0 }, stmt.pos);
                            self.gen_assign(dst, zero, func)?;
                        }
                    },
                }
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
            ExpressionKind::Ident { ref name } => {
                let var = self.ctx.find_variable(name);
                match var.1 {
                    Type::Array { .. } => {
                        let dst = self.gen_lvalue(expr, func)?;
                        Ok(dst)
                    }
                    _ => {
                        let src = self.gen_lvalue(expr, func)?;
                        let dst = self.next_reg();
                        func.push(IR::Load { dst, src });
                        Ok(dst)
                    }
                }
            }
            ExpressionKind::Index { .. } => {
                let dst = self.next_reg();
                let src = self.gen_lvalue(expr, func)?;
                func.push(IR::Load { dst, src });
                Ok(dst)
            }
            ExpressionKind::UnaryOp { op, expr } => match op {
                UnaryOperator::Addr => {
                    let src = self.gen_lvalue(*expr, func)?;
                    Ok(src)
                }
                UnaryOperator::Load => {
                    let src = self.gen_expression(*expr, func)?;
                    let dst = self.next_reg();
                    func.push(IR::Load { dst, src });
                    Ok(dst)
                }
                _ => {
                    let src = self.gen_expression(*expr, func)?;
                    func.push(IR::UnOp { op, src });
                    Ok(src)
                }
            },
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
        }
    }

    fn gen_lvalue(&mut self, expr: Expression, func: &mut IRFunction) -> Result<Operand, Error> {
        match expr.kind {
            ExpressionKind::Ident { name } => {
                let reg = self.next_reg();
                func.push(IR::Addr {
                    dst: reg,
                    src: self.ctx.find_variable(&name).0,
                });
                Ok(reg)
            }
            ExpressionKind::Index { lhs, index } => {
                let reg = self.gen_expression(*lhs, func)?;

                let index = self.gen_expression(*index, func)?;
                func.push(IR::BinOp {
                    op: BinaryOperator::Mul,
                    dst: index,
                    lhs: index,
                    rhs: Operand::Const(8), // TODO
                });
                func.push(IR::BinOp {
                    op: BinaryOperator::Add,
                    dst: reg,
                    lhs: reg,
                    rhs: index,
                });

                Ok(reg)
            }
            ExpressionKind::UnaryOp {
                op: UnaryOperator::Load,
                expr,
            } => {
                let reg = self.next_reg();
                let src = self.gen_expression(*expr, func)?;
                func.push(IR::Move { dst: reg, src });
                Ok(reg)
            }
            _ => panic!(),
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

    fn alloc_stack_local(&mut self, typ: &Type) -> MemoryAddr {
        self.stack_offset_local -= typ.size() as i32;
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
