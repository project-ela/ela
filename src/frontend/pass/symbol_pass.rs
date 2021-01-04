use crate::{
    common::{
        error::{Error, ErrorKind, Errors},
        operator::{BinaryOperator, UnaryOperator},
        pos::Pos,
        types::Type,
    },
    frontend::parser::ast::*,
};
use std::collections::HashMap;

struct SymbolPass<'a> {
    ctx: Context<'a>,
    issues: Errors,
}

struct Context<'a>(Vec<ContextData<'a>>);

#[derive(Default)]
struct ContextData<'a> {
    functions: HashMap<String, FunctionSig<'a>>,
    variables: HashMap<String, Variable>,
}

#[derive(Clone)]
struct FunctionSig<'a> {
    params: &'a Vec<Parameter>,
    ret_typ: Type,
}

#[derive(Clone)]
struct Variable {
    typ: Type,
    is_const: bool,
}

impl<'a> Context<'a> {
    fn new() -> Self {
        let mut ctx = Self(Vec::new());
        ctx.push();
        ctx
    }

    fn add_function(&mut self, name: String, params: &'a Vec<Parameter>, ret_typ: Type) {
        self.0
            .last_mut()
            .unwrap()
            .functions
            .insert(name, FunctionSig { params, ret_typ });
    }

    fn add_variable(&mut self, name: String, typ: Type, is_const: bool) {
        self.0
            .last_mut()
            .unwrap()
            .variables
            .insert(name, Variable { typ, is_const });
    }

    fn find_function(&self, name: &str) -> Option<&FunctionSig> {
        for ctx in self.0.iter().rev() {
            if ctx.functions.contains_key(name) {
                return ctx.functions.get(name);
            }
        }

        None
    }

    fn find_variable(&self, name: &str) -> Option<&Variable> {
        for ctx in self.0.iter().rev() {
            if ctx.variables.contains_key(name) {
                return ctx.variables.get(name);
            }
        }

        None
    }

    fn find_variable_scope(&self, name: &str) -> Option<&Variable> {
        let ctx = self.0.last().unwrap();
        ctx.variables.get(name)
    }

    fn push(&mut self) {
        self.0.push(ContextData::default());
    }

    fn pop(&mut self) {
        self.0.pop();
    }
}

pub fn apply(program: &Program) -> Result<(), Errors> {
    let mut pass = SymbolPass::new();
    pass.apply(program);

    let issues = pass.issues;
    if issues.0.is_empty() {
        Ok(())
    } else {
        Err(issues)
    }
}

impl<'a> SymbolPass<'a> {
    fn new() -> Self {
        Self {
            ctx: Context::new(),
            issues: Errors::default(),
        }
    }

    fn apply(&mut self, program: &'a Program) {
        if program.functions.iter().all(|f| f.name != "main") {
            self.issue(Error::new(Pos::default(), ErrorKind::MainNotFound));
        }

        for function in &program.functions {
            if function.name == "main" && function.ret_typ != Type::Int {
                self.issue(Error::new(
                    function.pos.clone(),
                    ErrorKind::MainShouldReturnInt,
                ));
            }
            self.apply_function(&function);
        }
    }

    fn apply_function(&mut self, function: &'a Function) {
        self.ctx.add_function(
            function.name.to_owned(),
            &function.params,
            function.ret_typ.clone(),
        );
        self.ctx.push();
        for param in &function.params {
            self.ctx
                .add_variable(param.name.to_owned(), param.typ.clone(), false);
        }
        if function.body.is_some() {
            self.apply_statement(function.body.as_ref().unwrap(), &function.ret_typ);
        }
        self.ctx.pop();
    }

    fn apply_statement(&mut self, stmt: &Statement, ret_typ: &Type) {
        match &stmt.kind {
            StatementKind::Block { stmts } => {
                self.ctx.push();
                for stmt in stmts {
                    self.apply_statement(&stmt, ret_typ);
                }
                self.ctx.pop();
            }
            StatementKind::Var { name, typ, value } => {
                self.apply_var(name, typ, value, &stmt.pos, false)
            }
            StatementKind::Val { name, typ, value } => {
                self.apply_var(name, typ, value, &stmt.pos, true)
            }
            StatementKind::Assign { dst, value } => {
                self.check_assign(&*dst, &*value, stmt.pos.clone())
            }
            StatementKind::Return { value } => {
                if let Some(value) = value {
                    if let Some(value_typ) = self.apply_expression(&*value) {
                        if &value_typ != ret_typ {
                            self.issue(Error::new(
                                stmt.pos.clone(),
                                ErrorKind::TypeMismatch {
                                    lhs: ret_typ.clone(),
                                    rhs: value_typ,
                                },
                            ));
                        }
                    }
                }
            }
            StatementKind::If { cond, then, els } => {
                match self.apply_expression(&*cond) {
                    Some(Type::Bool) | None => {}
                    Some(x) => self.issue(Error::new(
                        stmt.pos.clone(),
                        ErrorKind::TypeMismatch {
                            lhs: x,
                            rhs: Type::Bool,
                        },
                    )),
                }
                self.apply_statement(&*then, ret_typ);
                if let Some(els) = els {
                    self.apply_statement(&*els, ret_typ);
                }
            }
            StatementKind::While { cond, body } => {
                match self.apply_expression(&*cond) {
                    Some(Type::Bool) | None => {}
                    Some(x) => self.issue(Error::new(
                        stmt.pos.clone(),
                        ErrorKind::TypeMismatch {
                            lhs: x,
                            rhs: Type::Bool,
                        },
                    )),
                }
                self.apply_statement(&*body, ret_typ);
            }
            StatementKind::Call { name, args } => {
                self.check_call(&*name, args, stmt.pos.clone());
            }
        }
    }

    fn apply_var(
        &mut self,
        name: &String,
        typ: &Type,
        value: &Option<Box<Expression>>,
        pos: &Pos,
        is_const: bool,
    ) {
        if self.ctx.find_variable_scope(name).is_some() {
            self.issue(Error::new(
                pos.clone(),
                ErrorKind::RedefinitionOf { name: name.clone() },
            ));
        }

        self.ctx
            .add_variable(name.to_owned(), typ.clone(), is_const);

        let value_typ = value
            .as_deref()
            .and_then(|value| self.apply_expression(&*value));
        if let Some(value_typ) = value_typ {
            if &value_typ != typ {
                self.issue(Error::new(
                    pos.clone(),
                    ErrorKind::TypeMismatch {
                        lhs: typ.clone(),
                        rhs: value_typ,
                    },
                ));
            }
        }
    }

    fn apply_expression(&mut self, expr: &Expression) -> Option<Type> {
        use BinaryOperator::*;
        use UnaryOperator::*;
        match &expr.kind {
            ExpressionKind::Integer { .. } => Some(Type::Int),
            ExpressionKind::Bool { .. } => Some(Type::Bool),
            ExpressionKind::Ident { name } => match self.ctx.find_variable(&name) {
                Some(var) => Some(var.typ.clone()),
                None => {
                    self.issue(Error::new(
                        expr.pos.clone(),
                        ErrorKind::NotDefinedVariable { name: name.into() },
                    ));
                    None
                }
            },
            ExpressionKind::UnaryOp { op, expr } => {
                let expr_typ = self.apply_expression(&*expr)?;
                match op {
                    Not => match expr_typ {
                        Type::Bool => Some(Type::Bool),
                        typ => {
                            self.issue(Error::new(
                                expr.pos.clone(),
                                ErrorKind::UnaryOpErr { op: *op, expr: typ },
                            ));
                            None
                        }
                    },
                    Addr => match expr.kind {
                        ExpressionKind::Ident { .. }
                        | ExpressionKind::UnaryOp {
                            op: UnaryOperator::Addr,
                            ..
                        } => Some(Type::Pointer {
                            pointer_to: Box::new(expr_typ),
                        }),
                        _ => {
                            self.issue(Error::new(expr.pos.clone(), ErrorKind::LvalueRequired));
                            None
                        }
                    },
                    Load => match expr_typ {
                        Type::Pointer { pointer_to } => Some(*pointer_to),
                        x => {
                            self.issue(Error::new(
                                expr.pos.clone(),
                                ErrorKind::CannotLoad { lhs: x },
                            ));
                            None
                        }
                    },
                }
            }
            ExpressionKind::BinaryOp { op, lhs, rhs } => {
                let lhs_typ = self.apply_expression(&*lhs)?;
                let rhs_typ = self.apply_expression(&*rhs)?;
                if lhs_typ != rhs_typ {
                    self.issue(Error::new(
                        expr.pos.clone(),
                        ErrorKind::TypeMismatch {
                            lhs: lhs_typ,
                            rhs: rhs_typ,
                        },
                    ));
                    return None;
                }
                match op {
                    Equal | NotEqual | Lt | Lte | Gt | Gte => Some(Type::Bool),
                    Add | Sub | Mul | Div | And | Or | Xor => match lhs_typ {
                        Type::Int => Some(Type::Int),
                        _ => {
                            self.issue(Error::new(
                                expr.pos.clone(),
                                ErrorKind::BinaryOpErr {
                                    op: *op,
                                    lhs: lhs_typ,
                                    rhs: rhs_typ,
                                },
                            ));
                            None
                        }
                    },
                }
            }
            ExpressionKind::Call { name, args } => self.check_call(&name, args, expr.pos.clone()),
            ExpressionKind::Index { lhs, index } => {
                let lhs_typ = self.apply_expression(lhs)?;
                let index_typ = self.apply_expression(index)?;
                match lhs_typ {
                    Type::Array { elm_type: typ, .. } | Type::Pointer { pointer_to: typ } => {
                        if index_typ != Type::Int {
                            self.issue(Error::new(
                                expr.pos.clone(),
                                ErrorKind::TypeMismatch {
                                    lhs: index_typ,
                                    rhs: Type::Int,
                                },
                            ));
                        }
                        Some(*typ)
                    }
                    _ => {
                        self.issue(Error::new(
                            expr.pos.clone(),
                            ErrorKind::CannotIndex { lhs: lhs_typ },
                        ));
                        None
                    }
                }
            }
        }
    }

    fn check_assign(&mut self, dst: &Expression, value: &Expression, pos: Pos) {
        let dst_typ = self.apply_expression(dst);
        let value_typ = self.apply_expression(value);
        match (dst_typ, value_typ) {
            (Some(dst_typ), Some(value_typ)) => {
                if dst_typ != value_typ {
                    self.issue(Error::new(
                        pos.clone(),
                        ErrorKind::TypeMismatch {
                            lhs: dst_typ,
                            rhs: value_typ,
                        },
                    ));
                }
            }
            _ => return,
        }

        match &dst.kind {
            ExpressionKind::Ident { name } => {
                let var = self.ctx.find_variable(name).unwrap();
                if var.is_const {
                    self.issue(Error::new(
                        pos,
                        ErrorKind::AssignToConstant { name: name.into() },
                    ));
                }
            }
            ExpressionKind::Index { .. } => {}
            ExpressionKind::UnaryOp {
                op: UnaryOperator::Load,
                ..
            } => {}
            _ => self.issue(Error::new(pos, ErrorKind::LvalueRequired)),
        }
    }

    // TODO refactor
    fn check_call(&mut self, name: &str, args: &[Expression], pos: Pos) -> Option<Type> {
        let mut issues = Vec::new();

        let arg_types: Vec<Option<Type>> =
            args.iter().map(|arg| self.apply_expression(arg)).collect();

        let do_check = || {
            let sig = if let Some(sig) = self.ctx.find_function(name) {
                sig
            } else {
                self.issue(Error::new(
                    pos,
                    ErrorKind::NotDefinedFunction { name: name.into() },
                ));
                return None;
            };

            if args.len() != sig.params.len() {
                issues.push(Error::new(
                    pos.clone(),
                    ErrorKind::FunctionArgNum {
                        name: name.to_string(),
                        expected: sig.params.len(),
                        actual: args.len(),
                    },
                ));
                return Some(sig.ret_typ.clone());
            }

            let param_types = sig.params.iter().map(|param| param.typ.clone());

            for (arg_typ, param_typ) in arg_types.into_iter().zip(param_types) {
                match (&arg_typ, &param_typ) {
                    (Some(Type::Array { elm_type, .. }), Type::Pointer { pointer_to }) => {
                        if pointer_to == elm_type {
                            continue;
                        }
                    }
                    _ => {}
                }
                if let Some(arg_typ) = arg_typ {
                    if arg_typ != param_typ {
                        issues.push(Error::new(
                            pos.clone(),
                            ErrorKind::TypeMismatch {
                                lhs: arg_typ,
                                rhs: param_typ,
                            },
                        ));
                    }
                }
            }

            Some(sig.ret_typ.clone())
        };
        let ret_typ = do_check();

        issues.into_iter().for_each(|issue| self.issue(issue));
        ret_typ
    }

    fn issue(&mut self, err: Error) {
        self.issues.0.push(err);
    }
}
