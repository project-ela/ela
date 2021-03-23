use anyhow::Result;

use crate::{
    common::{
        error::{Error, Errors},
        operator::{BinOpType, BinaryOperator, UnaryOperator},
        pos::Pos,
        symtab::{NodeId, SigFunc, SigVar, SymbolTable},
        types::Type,
    },
    frontend::{
        ast::{Expression, ExpressionKind, Function, Parameter, Program, Statement, StatementKind},
        pass::error::PassError,
    },
};

pub fn apply(program: &Program) -> Result<SymbolTable> {
    let mut pass = TypeCheck::new();
    let table = pass.apply(program);
    match pass.issues.0.len() {
        0 => Ok(table),
        _ => Err(pass.issues.into()),
    }
}

#[derive(Debug)]
struct TypeCheck {
    nodes: Vec<NodeId>,
    table: SymbolTable,
    issues: Errors,

    cur_ret_typ: Option<Type>,
    cur_pos: Option<Pos>,
}

impl TypeCheck {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            table: SymbolTable::new(),
            issues: Errors::default(),

            cur_ret_typ: None,
            cur_pos: None,
        }
    }

    fn apply(&mut self, program: &Program) -> SymbolTable {
        self.push(program.id);

        for global_def in &program.global_defs {
            self.add_var(
                global_def.name.clone(),
                global_def.typ.clone(),
                global_def.is_const,
            );
        }

        for function in &program.functions {
            self.apply_function(function);
        }
        self.pop();

        std::mem::take(&mut self.table)
    }

    fn apply_function(&mut self, function: &Function) {
        self.add_func(
            function.name.clone(),
            function.params.to_owned(),
            function.ret_typ.clone(),
        );

        self.push(function.id);

        self.cur_ret_typ = Some(function.ret_typ.clone());
        self.cur_pos = Some(function.pos.clone());

        for param in &function.params {
            self.add_var(param.name.clone(), param.typ.clone(), true);
        }

        if let Some(body) = &function.body {
            self.apply_stmt(body);
        }

        self.pop();
    }

    fn apply_stmt(&mut self, stmt: &Statement) {
        self.cur_pos = Some(stmt.pos.clone());
        match &stmt.kind {
            StatementKind::Block { stmts } => {
                self.push(stmt.id);
                for stmt in stmts {
                    self.apply_stmt(stmt);
                }
                self.pop();
            }
            StatementKind::Var { name, typ, value } => {
                self.apply_var_stmt(name, typ, value.as_deref())
            }
            StatementKind::Val { name, typ, value } => {
                self.apply_val_stmt(name, typ, value.as_deref())
            }
            StatementKind::Assign { dst, value } => self.apply_assign_stmt(dst, value),
            StatementKind::Return { value } => self.apply_return_stmt(value.as_deref()),
            StatementKind::If { cond, then, els } => self.apply_if_stmt(cond, then, els.as_deref()),
            StatementKind::While { cond, body } => self.apply_while_stmt(cond, body),
            StatementKind::Call { name, args } => {
                self.apply_call(name, args);
            }
        }
    }

    fn apply_var_stmt(&mut self, name: &String, typ: &Type, value: Option<&Expression>) {
        self.add_var(name.clone(), typ.clone(), false);
        if let Some(value) = value {
            self.check_expr_type(value, typ.clone());
        }
    }

    fn apply_val_stmt(&mut self, name: &String, typ: &Type, value: Option<&Expression>) {
        self.add_var(name.clone(), typ.clone(), true);
        if let Some(value) = value {
            self.check_expr_type(value, typ.clone());
        }
    }

    fn apply_assign_stmt(&mut self, dst: &Expression, value: &Expression) {
        let dst_typ = match self.apply_expr(dst) {
            Some(typ) => typ,
            None => return,
        };
        if self.check_expr_type(value, dst_typ).is_none() {
            return;
        }

        if let ExpressionKind::Ident { name } = &dst.kind {
            let SigVar(_, is_const) = self.table.find_variable(self.cur_node(), name).unwrap();
            if is_const {
                self.issue_here::<()>(PassError::AssignToConstant(name.clone()));
            }
        }

        if !Self::is_lvalue(&dst.kind) {
            self.issue_here::<()>(PassError::LvalueRequired);
        }
    }

    fn is_lvalue(kind: &ExpressionKind) -> bool {
        match kind {
            ExpressionKind::Ident { .. } => true,
            ExpressionKind::Index { .. } => true,
            ExpressionKind::UnaryOp {
                op: UnaryOperator::Load,
                ..
            } => true,
            _ => false,
        }
    }

    fn apply_return_stmt(&mut self, value: Option<&Expression>) {
        let value = match value {
            Some(value) => value,
            None => return,
        };

        self.check_expr_type(value, self.cur_ret_typ.clone().unwrap());
    }

    fn apply_if_stmt(&mut self, cond: &Expression, then: &Statement, els: Option<&Statement>) {
        self.check_expr_type(cond, Type::Bool);
        self.apply_stmt(then);
        if let Some(els) = els {
            self.apply_stmt(els);
        }
    }

    fn apply_while_stmt(&mut self, cond: &Expression, body: &Statement) {
        self.check_expr_type(cond, Type::Bool);
        self.apply_stmt(body);
    }

    fn apply_expr(&mut self, expr: &Expression) -> Option<Type> {
        self.cur_pos = Some(expr.pos.clone());
        match &expr.kind {
            ExpressionKind::Char { .. } => Some(Type::Byte),
            ExpressionKind::Integer { .. } => Some(Type::Int),
            ExpressionKind::String { .. } => Some(Type::Byte.pointer_to()),
            ExpressionKind::Bool { .. } => Some(Type::Bool),

            ExpressionKind::Ident { name } => self.apply_ident_expr(name),
            ExpressionKind::UnaryOp { op, expr } => self.apply_unop_expr(op, expr),
            ExpressionKind::BinaryOp { op, lhs, rhs } => self.apply_binop_expr(op, lhs, rhs),
            ExpressionKind::Call { name, args } => self.apply_call(name, args),
            ExpressionKind::Index { lhs, index } => self.apply_index_expr(lhs, index),
        }
    }

    fn apply_ident_expr(&mut self, name: &String) -> Option<Type> {
        match self.table.find_variable(self.cur_node(), name) {
            Some(SigVar(typ, _)) => Some(typ),
            None => self.issue_here(PassError::NotDefinedVariable(name.clone())),
        }
    }

    fn apply_unop_expr(&mut self, op: &UnaryOperator, expr: &Expression) -> Option<Type> {
        let expr_typ = self.apply_expr(expr)?;
        match op {
            UnaryOperator::Not => match expr_typ {
                x @ Type::Bool => Some(x),
                x => self.issue_here(PassError::UnaryOpErr(op.clone(), x)),
            },
            UnaryOperator::Addr => match expr.kind {
                ExpressionKind::Ident { .. } => Some(expr_typ.pointer_to()),
                _ => self.issue_here(PassError::LvalueRequired),
            },
            UnaryOperator::Load => match expr_typ {
                Type::Pointer { pointer_to } => Some(*pointer_to),
                x => self.issue_here(PassError::CannotLoad(x)),
            },
        }
    }

    fn apply_binop_expr(
        &mut self,
        op: &BinaryOperator,
        lhs: &Expression,
        rhs: &Expression,
    ) -> Option<Type> {
        let lhs_typ = self.apply_expr(lhs)?;
        let rhs_typ = self.apply_expr(rhs)?;
        if !lhs_typ.is_same(&rhs_typ) {
            self.issue_here(PassError::TypeMismatch(lhs_typ.clone(), rhs_typ.clone()))?;
        }

        match op.typ() {
            BinOpType::Comparison => match lhs_typ {
                Type::Byte | Type::Int | Type::Bool => Some(Type::Bool),
                _ => self.issue_here(PassError::BinaryOpErr(op.clone(), lhs_typ, rhs_typ)),
            },
            BinOpType::Arithmetic => match lhs_typ {
                x @ Type::Byte | x @ Type::Int => Some(x),
                _ => self.issue_here(PassError::BinaryOpErr(op.clone(), lhs_typ, rhs_typ)),
            },
        }
    }

    fn apply_index_expr(&mut self, lhs: &Expression, index: &Expression) -> Option<Type> {
        let lhs_typ = self.apply_expr(lhs)?;
        let index_typ = self.apply_expr(index)?;

        if !index_typ.is_same(&Type::Int) {
            self.issue_here::<()>(PassError::TypeMismatch(index_typ, Type::Int));
        }

        match lhs_typ {
            Type::Array { elm_type: typ, .. } | Type::Pointer { pointer_to: typ } => Some(*typ),
            x => self.issue_here(PassError::CannotIndex(x)),
        }
    }

    fn apply_call(&mut self, name: &String, args: &[Expression]) -> Option<Type> {
        let SigFunc(params, ret_typ) = self.table.find_function(self.cur_node(), name)?;

        if args.len() != params.len() {
            self.issue_here::<()>(PassError::FunctionArgNum(
                name.clone(),
                params.len(),
                args.len(),
            ));
            return Some(ret_typ);
        }

        for (arg, param) in args.iter().zip(&params) {
            self.check_expr_type(arg, param.typ.clone());
        }

        Some(ret_typ)
    }

    // -----

    fn check_expr_type(&mut self, expr: &Expression, expected: Type) -> Option<Type> {
        let expr_typ = self.apply_expr(expr)?;
        if !expr_typ.is_same(&expected) {
            self.issue::<()>(
                expr.pos.clone(),
                PassError::TypeMismatch(expr_typ.clone(), expected),
            );
        }

        Some(expr_typ)
    }

    fn add_var(&mut self, name: String, typ: Type, is_const: bool) {
        if self.table.is_defined_here(self.cur_node(), &name) {
            self.issue_here::<()>(PassError::RedefinitionOf(name.clone()));
        }

        self.table
            .add_variable(self.cur_node(), name, SigVar(typ, is_const));
    }

    fn add_func(&mut self, name: String, params: Vec<Parameter>, ret_typ: Type) {
        self.table
            .add_function(self.cur_node(), name, SigFunc(params, ret_typ));
    }

    fn issue<T>(&mut self, pos: Pos, err: PassError) -> Option<T> {
        self.issues.0.push(Error::new(pos, err));
        None
    }

    fn issue_here<T>(&mut self, err: PassError) -> Option<T> {
        self.issue(self.cur_pos.clone().unwrap(), err)
    }

    fn push(&mut self, node: NodeId) {
        let parent_node = self.nodes.last().cloned();
        self.table.add_scope(node, parent_node);

        self.nodes.push(node);
    }

    fn pop(&mut self) {
        self.nodes.pop().unwrap();
    }

    fn cur_node(&self) -> NodeId {
        self.nodes.last().unwrap().clone()
    }
}
