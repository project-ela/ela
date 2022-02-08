use crate::{
    common::{
        operator::{BinaryOperator, UnaryOperator},
        symtab::{NodeId, SymbolTable},
        types::Type,
    },
    frontend::ast,
};
use anyhow::Result;
use ir::{RegSize, IR};

use super::ir::{self, Operand, RegisterInfo, Tse};

#[derive(Debug)]
struct IRGen<'a> {
    symtab: &'a mut SymbolTable,
    cur_nodes: Vec<NodeId>,

    cur_funcname: String,
    global_vars: Vec<ir::GlobalVar>,

    reg_index: u32,
    label_index: u32,
    string_index: u32,
    stack_offset_local: u32,
}

pub fn generate(module: ast::Module, symtab: &mut SymbolTable) -> Result<ir::Module> {
    let mut generator = IRGen::new(symtab);
    Ok(generator.generate(module)?)
}

impl<'a> IRGen<'a> {
    fn new(symtab: &'a mut SymbolTable) -> Self {
        Self {
            symtab,
            cur_nodes: Vec::new(),

            cur_funcname: "".into(),
            global_vars: Vec::new(),

            reg_index: 0,
            label_index: 0,
            string_index: 0,
            stack_offset_local: 0,
        }
    }

    fn generate(&mut self, module: ast::Module) -> Result<ir::Module> {
        let mut ir_module = ir::Module::default();
        for global_var in module.global_vars {
            ir_module.global_vars.push(ir::GlobalVar {
                name: global_var.name.clone(),
                typ: global_var.typ.clone(),
                init_value: None,
            });
        }
        for function in module.functions {
            if let Some(func) = self.gen_function(function)? {
                ir_module.functions.push(func);
            }
        }

        ir_module
            .global_vars
            .extend(std::mem::take(&mut self.global_vars));

        Ok(ir_module)
    }

    fn gen_function(&mut self, func: ast::Function) -> Result<Option<ir::Function>> {
        if func.body.is_none() {
            return Ok(None);
        }

        self.init();
        self.push(func.id);
        self.cur_funcname = func.name.clone();
        let mut ir_func = ir::Function::new(func.name.to_owned());
        ir_func.new_block(format!(".L.{}.entry", func.name));
        for (index, param) in func.params.iter().enumerate() {
            let addr = self.alloc_stack_local(&param.typ, &mut ir_func);
            let size = RegSize::from(&param.typ);
            self.symtab
                .set_local(self.cur_node(), param.name.clone(), addr);
            ir_func.push(IR::StoreArg {
                dst: addr,
                src: index,
                size,
            });
            ir_func.params.push(index as u32);
        }
        self.gen_statement(func.body.unwrap(), &mut ir_func)?;
        ir_func.stack_offset = align_to(self.stack_offset_local, 8);
        self.pop();
        Ok(Some(ir_func))
    }

    fn gen_statement(&mut self, stmt: ast::Statement, func: &mut ir::Function) -> Result<()> {
        match stmt.kind {
            ast::StatementKind::Block { stmts } => {
                self.push(stmt.id);
                for stmt in stmts {
                    self.gen_statement(stmt, func)?;
                }
                self.pop();
            }
            ast::StatementKind::Var { name, typ, value }
            | ast::StatementKind::Val { name, typ, value } => {
                let addr = self.alloc_stack_local(&typ, func);
                self.symtab.set_local(self.cur_node(), name, addr);

                match value {
                    Some(value) => {
                        let dst = self.next_reg();
                        func.push(IR::Addr { dst, src: addr });
                        self.gen_assign(dst, &typ, *value, func)?;
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

                                let zero = ast::Expression::new(
                                    ast::ExpressionKind::Integer { value: 0 },
                                    stmt.pos.clone(),
                                );
                                self.gen_assign(dst, &elm_type, zero, func)?;
                            }
                        }
                        _ => {
                            let dst = self.next_reg();
                            func.push(IR::Addr { dst, src: addr });
                            let zero = ast::Expression::new(
                                ast::ExpressionKind::Integer { value: 0 },
                                stmt.pos,
                            );
                            self.gen_assign(dst, &typ, zero, func)?;
                        }
                    },
                }
            }
            ast::StatementKind::Assign { dst, value } => {
                let (dst, typ) = self.gen_lvalue(*dst, func)?;
                self.gen_assign(dst, &typ.elm_typ(), *value, func)?;
            }
            ast::StatementKind::Return { value } => {
                let src = match value {
                    Some(value) => Some(self.gen_expression(*value, func)?.0),
                    None => None,
                };
                func.push(IR::Ret { src });
            }
            ast::StatementKind::If { cond, then, els } => {
                let label1 = self.next_label();

                let (cond, _) = self.gen_expression(*cond, func)?;
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
            ast::StatementKind::While { cond, body } => {
                let label1 = self.next_label();
                let label2 = self.next_label();

                // condition
                func.new_block(label1.to_owned());
                let (cond, _) = self.gen_expression(*cond, func)?;
                func.push(IR::JumpIfNot {
                    label: label2.to_owned(),
                    cond,
                });

                // body
                self.gen_statement(*body, func)?;
                func.push(IR::Jump { label: label1 });

                func.new_block(label2);
            }
            ast::StatementKind::Call { name, args } => {
                let _ = self.gen_call(None, name, args, func)?;
            }
        }
        Ok(())
    }

    fn gen_expression(
        &mut self,
        expr: ast::Expression,
        func: &mut ir::Function,
    ) -> Result<(Operand, Type)> {
        let val = match expr.kind {
            ast::ExpressionKind::Char { value } => {
                let dst = self.next_reg();
                func.push(IR::Move {
                    dst,
                    src: Operand::Const(value as i32),
                });
                Ok((dst, Type::Byte))
            }
            ast::ExpressionKind::Integer { value } => {
                let dst = self.next_reg();
                func.push(IR::Move {
                    dst,
                    src: Operand::Const(value),
                });
                Ok((dst, Type::Int))
            }
            ast::ExpressionKind::String { value } => {
                let name = self.next_string();
                self.global_vars.push(ir::GlobalVar {
                    name: name.clone(),
                    typ: Type::Byte.pointer_to(),
                    init_value: Some(value),
                });

                let dst = self.next_reg();
                func.push(IR::AddrLabel { dst, src: name });
                Ok((dst, Type::Byte.pointer_to()))
            }
            ast::ExpressionKind::Bool { value } => {
                let dst = self.next_reg();
                func.push(IR::Move {
                    dst,
                    src: Operand::Const(value as i32),
                });
                Ok((dst, Type::Bool))
            }
            ast::ExpressionKind::Ident { ref name } => {
                let sig = self.symtab.find_variable(self.cur_node(), name).unwrap();
                match sig.typ {
                    Type::Array { .. } => {
                        let (dst, _) = self.gen_lvalue(expr, func)?;
                        Ok((dst, sig.typ))
                    }
                    _ => {
                        let (src, _) = self.gen_lvalue(expr, func)?;
                        let dst = self.next_reg();
                        let size = RegSize::from(&sig.typ);
                        func.push(IR::Load { dst, src, size });
                        Ok((dst, sig.typ))
                    }
                }
            }
            ast::ExpressionKind::Index { .. } => {
                let dst = self.next_reg();
                let (src, typ) = self.gen_lvalue(expr, func)?;
                let typ = typ.elm_typ();
                let size = RegSize::from(&typ);
                func.push(IR::Load { dst, src, size });
                Ok((dst, typ))
            }
            ast::ExpressionKind::UnaryOp { op, expr } => match op {
                UnaryOperator::Addr => {
                    let (src, typ) = self.gen_lvalue(*expr, func)?;
                    Ok((src, typ))
                }
                UnaryOperator::Load => {
                    let (src, typ) = self.gen_expression(*expr, func)?;
                    let typ = typ.elm_typ();
                    let dst = self.next_reg();
                    let size = RegSize::from(&typ);
                    func.push(IR::Load { dst, src, size });
                    Ok((dst, typ))
                }
                _ => {
                    let (src, typ) = self.gen_expression(*expr, func)?;
                    func.push(IR::UnOp { op, src });
                    Ok((src, typ))
                }
            },
            ast::ExpressionKind::BinaryOp { op, lhs, rhs } => {
                let (lhs, typ) = self.gen_expression(*lhs, func)?;
                let (rhs, _) = self.gen_expression(*rhs, func)?;
                let dst = self.next_reg();
                func.push(IR::BinOp { op, dst, lhs, rhs });
                Ok((dst, typ))
            }
            ast::ExpressionKind::Call { name, args } => {
                let dst = self.next_reg();
                let typ = self.gen_call(Some(dst), name, args, func)?;
                Ok((dst, typ))
            }
        };
        val
    }

    fn gen_lvalue(
        &mut self,
        expr: ast::Expression,
        func: &mut ir::Function,
    ) -> Result<(Operand, Type)> {
        match expr.kind {
            ast::ExpressionKind::Ident { name } => {
                let reg = self.next_reg();
                let sig = self.symtab.find_variable(self.cur_node(), &name).unwrap();
                let ir = if let Some(offset) = sig.offset {
                    IR::Addr {
                        dst: reg,
                        src: offset,
                    }
                } else {
                    IR::AddrLabel {
                        dst: reg,
                        src: name,
                    }
                };
                func.push(ir);
                Ok((reg, sig.typ.pointer_to()))
            }
            ast::ExpressionKind::Index { lhs, index } => {
                let (reg, typ) = self.gen_expression(*lhs, func)?;

                let (index, _) = self.gen_expression(*index, func)?;
                func.push(IR::BinOp {
                    op: BinaryOperator::Mul,
                    dst: index,
                    lhs: index,
                    rhs: Operand::Const(typ.elm_typ().size() as i32),
                });
                func.push(IR::BinOp {
                    op: BinaryOperator::Add,
                    dst: reg,
                    lhs: reg,
                    rhs: index,
                });

                Ok((reg, typ))
            }
            ast::ExpressionKind::UnaryOp {
                op: UnaryOperator::Load,
                expr,
            } => {
                let reg = self.next_reg();
                let (src, typ) = self.gen_expression(*expr, func)?;
                func.push(IR::Move { dst: reg, src });
                Ok((reg, typ))
            }
            _ => panic!(),
        }
    }

    fn gen_assign(
        &mut self,
        dst: Operand,
        dst_typ: &Type,
        src: ast::Expression,
        func: &mut ir::Function,
    ) -> Result<()> {
        let (src, _) = self.gen_expression(src, func)?;
        let size = RegSize::from(dst_typ);
        func.push(IR::Store { dst, src, size });
        Ok(())
    }

    fn gen_call(
        &mut self,
        dst: Option<Operand>,
        name: String,
        args: Vec<ast::Expression>,
        func: &mut ir::Function,
    ) -> Result<Type> {
        let mut arg_operands = Vec::new();
        for arg in args {
            arg_operands.push(self.gen_expression(arg, func)?.0);
        }
        let sig = self.symtab.find_function(self.cur_node(), &name).unwrap();
        func.push(IR::Call {
            dst,
            name,
            args: arg_operands,
        });
        Ok(sig.ret_typ)
    }

    fn init(&mut self) {
        self.reg_index = 0;
        self.label_index = 0;
        self.stack_offset_local = 0;
    }

    fn next_reg(&mut self) -> Operand {
        let cur_reg = self.reg_index;
        self.reg_index += 1;
        Operand::Reg(RegisterInfo {
            virtual_index: cur_reg,
            physical_index: None,
        })
    }

    fn next_label(&mut self) -> String {
        let cur_label = self.label_index;
        self.label_index += 1;
        format!(".L.{}.{}", self.cur_funcname, cur_label)
    }

    fn next_string(&mut self) -> String {
        let cur_string = self.string_index;
        self.string_index += 1;
        format!(".str.{}", cur_string)
    }

    fn alloc_stack_local(&mut self, typ: &Type, func: &mut ir::Function) -> i32 {
        self.stack_offset_local += typ.size();
        self.stack_offset_local = align_to(self.stack_offset_local, typ.size());
        let offset = -(self.stack_offset_local as i32);

        // gen TSE
        {
            let size = typ.size();
            let align = match typ {
                Type::Pointer { .. } | Type::Array { .. } => typ.elm_typ().size(),
                x => x.size(),
            };
            func.tses.push(Tse {
                offset: offset as i64,
                size: size as u64,
                align: align as u64,
            });
        }

        offset
    }

    fn push(&mut self, node: NodeId) {
        self.cur_nodes.push(node);
    }

    fn pop(&mut self) {
        self.cur_nodes.pop();
    }

    fn cur_node(&self) -> NodeId {
        self.cur_nodes.last().unwrap().clone()
    }
}

fn align_to(x: u32, align: u32) -> u32 {
    (x + align - 1) & !(align - 1)
}

impl ir::Function {
    fn push(&mut self, ir: IR) {
        let last_block = self.blocks.last_mut().unwrap();
        last_block.irs.push(ir);
    }

    fn new_block(&mut self, name: String) {
        self.blocks.push(ir::Block::new(name));
    }
}
