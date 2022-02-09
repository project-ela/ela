use siderow::ssa;

use crate::common::{
    operator::{BinaryOperator, UnaryOperator},
    symtab::{NodeId, SymbolTable},
    types::Type,
};
use crate::frontend::ast;

pub fn translate(module: ast::Module, symtab: &mut SymbolTable) -> ssa::Module {
    SsaGen::new(symtab).translate(module)
}

struct SsaGen<'a> {
    module: ssa::Module,

    symtab: &'a mut SymbolTable,
    scopes: Vec<NodeId>,
    string_index: usize,
}

impl<'a> SsaGen<'a> {
    fn new(symtab: &'a mut SymbolTable) -> Self {
        Self {
            module: ssa::Module::new(),
            symtab,
            scopes: Vec::new(),
            string_index: 0,
        }
    }

    fn translate(mut self, module: ast::Module) -> ssa::Module {
        self.push(module.id);
        for global in module.global_vars {
            self.trans_global(global);
        }

        for function in module.functions {
            self.trans_function(function);
        }
        self.pop();

        self.module
    }

    fn trans_global(&mut self, ast_global: ast::GlobalVar) {
        let global_name = ast_global.name;
        let global_typ = Self::trans_type(ast_global.typ);
        let ssa_global =
            ssa::Global::new(global_name.clone(), global_typ, ssa::Constant::new_zero());
        let global_id = self.module.add_global(ssa_global);

        let dst = ssa::Value::new_global(&mut self.module, global_id);
        self.symtab.set_local(self.cur_scope(), global_name, dst);
    }

    fn trans_function(&mut self, func: ast::Function) {
        let ret_typ = Self::trans_type(func.ret_typ);
        let param_typ = func
            .params
            .iter()
            .map(|param| Self::trans_type(param.typ.clone()))
            .collect();

        let function = ssa::Function::new(&func.name, ret_typ, param_typ);
        let func_name = func.name.clone();
        let func_id = self.module.add_function(function);
        self.symtab.set_id(self.cur_scope(), func_name, func_id);

        if func.body.is_none() {
            return;
        }

        // TODO
        let dummy_function = ssa::Function::new("", ssa::Type::Void, vec![]);
        let mut ssa_function =
            std::mem::replace(self.module.function_mut(func_id).unwrap(), dummy_function);
        let mut builder = ssa::FunctionBuilder::new(&mut ssa_function);
        let entry_block = builder.new_block();
        builder.set_block(entry_block);

        self.push(func.id);

        for (i, param) in func.params.iter().enumerate() {
            let name = param.name.clone();
            let val = ssa::Value::new_param(builder.function(), i);
            let dst = builder.alloc(val.typ());
            builder.store(dst.clone(), val);
            self.symtab.set_local(self.cur_scope(), name, dst);
        }

        if let Some(body) = func.body {
            self.trans_stmt(body, &mut builder);
        }

        self.pop();

        let _ = std::mem::replace(self.module.function_mut(func_id).unwrap(), ssa_function);
    }

    fn trans_stmt(&mut self, stmt: ast::Statement, builder: &mut ssa::FunctionBuilder) {
        match stmt.kind {
            ast::StatementKind::Block { stmts } => {
                self.push(stmt.id);
                for stmt in stmts {
                    let stop_translation = matches!(stmt.kind, ast::StatementKind::Return { .. });
                    self.trans_stmt(stmt, builder);
                    if stop_translation {
                        break;
                    }
                }
                self.pop();
            }
            ast::StatementKind::Var { name, typ, value }
            | ast::StatementKind::Val { name, typ, value } => {
                self.trans_var(name, typ, value.map(|v| *v), builder)
            }
            ast::StatementKind::Assign { dst, value } => self.trans_assign(*dst, *value, builder),
            ast::StatementKind::Return { value } => {
                self.trans_return_stmt(value.map(|v| *v), builder)
            }
            ast::StatementKind::If { cond, then, els } => {
                self.trans_if_stmt(*cond, *then, els.map(|v| *v), builder)
            }
            ast::StatementKind::While { cond, body } => {
                self.trans_while_stmt(*cond, *body, builder)
            }
            ast::StatementKind::Call { name, args } => {
                self.trans_call(name, args, builder);
            }
        }
    }

    fn trans_var(
        &mut self,
        name: String,
        typ: Type,
        value: Option<ast::Expression>,
        builder: &mut ssa::FunctionBuilder,
    ) {
        let typ = Self::trans_type(typ);
        let dst = builder.alloc(typ);

        let src = match value {
            Some(value) => self.trans_expr(value, builder),
            None => ssa::Value::new_zero(),
        };
        builder.store(dst.clone(), src);

        self.symtab.set_local(self.cur_scope(), name, dst);
    }

    fn trans_assign(
        &mut self,
        dst: ast::Expression,
        value: ast::Expression,
        builder: &mut ssa::FunctionBuilder,
    ) {
        let dst = self.trans_lvalue(dst, builder);
        let src = self.trans_expr(value, builder);
        builder.store(dst, src);
    }

    fn trans_return_stmt(
        &mut self,
        value: Option<ast::Expression>,
        builder: &mut ssa::FunctionBuilder,
    ) {
        match value {
            None => builder.ret_void(),
            Some(value) => {
                let value = self.trans_expr(value, builder);
                builder.ret(value);
            }
        }
    }

    fn trans_if_stmt(
        &mut self,
        cond: ast::Expression,
        then: ast::Statement,
        els: Option<ast::Statement>,
        builder: &mut ssa::FunctionBuilder,
    ) {
        let block_then = builder.new_block();
        let block_els = builder.new_block();
        let block_merge = if els.is_some() {
            builder.new_block()
        } else {
            block_els
        };

        let cond = self.trans_expr(cond, builder);
        builder.cond_br(cond, block_then, block_els);

        builder.set_block(block_then);
        self.trans_stmt(then, builder);
        if !builder.is_terminated() {
            builder.br(block_merge);
        }

        builder.set_block(block_els);
        if let Some(els) = els {
            self.trans_stmt(els, builder);
            if !builder.is_terminated() {
                builder.br(block_merge);
            }
        }

        builder.set_block(block_merge);
    }

    fn trans_while_stmt(
        &mut self,
        cond: ast::Expression,
        body: ast::Statement,
        builder: &mut ssa::FunctionBuilder,
    ) {
        let cond_block = builder.new_block();
        let body_block = builder.new_block();
        let exit_block = builder.new_block();

        builder.br(cond_block);
        builder.set_block(cond_block);
        let cond = self.trans_expr(cond, builder);
        builder.cond_br(cond, body_block, exit_block);

        builder.set_block(body_block);
        self.trans_stmt(body, builder);
        builder.br(cond_block);

        builder.set_block(exit_block)
    }

    fn trans_expr(
        &mut self,
        expr: ast::Expression,
        builder: &mut ssa::FunctionBuilder,
    ) -> ssa::Value {
        match expr.kind {
            ast::ExpressionKind::Bool { value } => ssa::Value::new_i1(value),
            ast::ExpressionKind::Char { value } => ssa::Value::new_i8(value as i8),
            ast::ExpressionKind::Integer { value } => ssa::Value::new_i32(value),
            ast::ExpressionKind::String { value } => self.trans_string(value, builder),

            ast::ExpressionKind::Ident { name } => self.trans_ident(name, builder),
            ast::ExpressionKind::UnaryOp { op, expr } => self.trans_unop(op, *expr, builder),
            ast::ExpressionKind::BinaryOp { op, lhs, rhs } => {
                self.trans_binop(op, *lhs, *rhs, builder)
            }
            ast::ExpressionKind::Call { name, args } => self.trans_call(name, args, builder),
            ast::ExpressionKind::Index { .. } => self.trans_index(expr, builder),
        }
    }

    fn trans_string(
        &mut self,
        mut value: String,
        builder: &mut ssa::FunctionBuilder,
    ) -> ssa::Value {
        value.push('\0');
        let str_bytes = ssa::Constant::new_array_from_bytes(value.as_bytes());

        let str_name = self.next_string_name();
        let str_global = ssa::Global::new(str_name, str_bytes.typ(), str_bytes);
        let str_id = self.module.add_global(str_global);

        builder.gep(
            ssa::Value::new_global(&mut self.module, str_id),
            vec![ssa::Value::new_i32(0), ssa::Value::new_i32(0)],
        )
    }

    fn trans_ident(&mut self, name: String, builder: &mut ssa::FunctionBuilder) -> ssa::Value {
        let sig = self.symtab.find_variable(self.cur_scope(), &name).unwrap();
        match sig.typ {
            Type::Array { .. } => builder.gep(
                sig.val.unwrap(),
                vec![ssa::Value::new_i32(0), ssa::Value::new_i32(0)],
            ),
            _ => builder.load(sig.val.unwrap()),
        }
    }

    fn trans_unop(
        &mut self,
        op: UnaryOperator,
        expr: ast::Expression,
        builder: &mut ssa::FunctionBuilder,
    ) -> ssa::Value {
        use UnaryOperator::*;

        match op {
            Addr => self.trans_lvalue(expr, builder),
            Load => {
                let expr = self.trans_expr(expr, builder);
                builder.load(expr)
            }
            Not => {
                let expr = self.trans_expr(expr, builder);
                builder.xor(expr, ssa::Value::new_i1(true))
            }
        }
    }

    fn trans_binop(
        &mut self,
        op: BinaryOperator,
        lhs: ast::Expression,
        rhs: ast::Expression,
        builder: &mut ssa::FunctionBuilder,
    ) -> ssa::Value {
        use BinaryOperator::*;

        let lhs = self.trans_expr(lhs, builder);
        let rhs = self.trans_expr(rhs, builder);

        match op {
            Add => builder.add(lhs, rhs),
            Sub => builder.sub(lhs, rhs),
            Mul => builder.mul(lhs, rhs),
            Div => builder.div(lhs, rhs),
            Mod => builder.rem(lhs, rhs),
            And => builder.and(lhs, rhs),
            Or => builder.or(lhs, rhs),
            Xor => builder.xor(lhs, rhs),

            Equal => builder.eq(lhs, rhs),
            NotEqual => builder.neq(lhs, rhs),
            Lt => builder.lt(lhs, rhs),
            Lte => builder.lte(lhs, rhs),
            Gt => builder.gt(lhs, rhs),
            Gte => builder.gte(lhs, rhs),
        }
    }

    fn trans_call(
        &mut self,
        name: String,
        args: Vec<ast::Expression>,
        builder: &mut ssa::FunctionBuilder,
    ) -> ssa::Value {
        let sig = self.symtab.find_function(self.cur_scope(), &name).unwrap();
        let args = args
            .into_iter()
            .map(|arg| self.trans_expr(arg, builder))
            .collect();
        builder.call(&self.module, sig.id.unwrap(), args)
    }

    fn trans_index(
        &mut self,
        expr: ast::Expression,
        builder: &mut ssa::FunctionBuilder,
    ) -> ssa::Value {
        let indexed_expr = self.trans_lvalue(expr, builder);
        builder.load(indexed_expr)
    }

    fn trans_lvalue(
        &mut self,
        expr: ast::Expression,
        builder: &mut ssa::FunctionBuilder,
    ) -> ssa::Value {
        match expr.kind {
            ast::ExpressionKind::Ident { name } => {
                let sig = self.symtab.find_variable(self.cur_scope(), &name).unwrap();
                sig.val.unwrap()
            }
            ast::ExpressionKind::Index { lhs, index } => {
                let lhs = self.trans_expr(*lhs, builder);
                let index = self.trans_expr(*index, builder);

                let elm_typ = lhs.typ().elm_typ();
                match elm_typ {
                    ssa::Type::Array(_, _) => builder.gep(lhs, vec![ssa::Value::new_i32(0), index]),
                    _ => builder.gep(lhs, vec![index]),
                }
            }
            ast::ExpressionKind::UnaryOp {
                op: UnaryOperator::Load,
                expr,
            } => {
                let expr = self.trans_lvalue(*expr, builder);
                builder.load(expr)
            }
            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_type(typ: Type) -> ssa::Type {
        match typ {
            Type::Void => ssa::Type::Void,
            Type::Bool => ssa::Type::I1,
            Type::Byte => ssa::Type::I8,
            Type::Int => ssa::Type::I32,
            Type::Pointer { pointer_to } => Self::trans_type(*pointer_to).ptr_to(),
            Type::Array { elm_type, len } => Self::trans_type(*elm_type).array_of(len as usize),
        }
    }

    fn next_string_name(&mut self) -> String {
        self.string_index += 1;
        format!(".str.{}", self.string_index)
    }

    fn push(&mut self, node: NodeId) {
        self.scopes.push(node);
    }

    fn pop(&mut self) {
        self.scopes.pop();
    }

    fn cur_scope(&self) -> NodeId {
        *self.scopes.last().unwrap()
    }
}
