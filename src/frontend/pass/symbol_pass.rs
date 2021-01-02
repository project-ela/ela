use crate::{
    common::{
        error::{Error, ErrorKind, Errors},
        operator::BinaryOperator,
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
        self.ctx
            .add_function(function.name.to_owned(), &function.params, function.ret_typ);
        self.ctx.push();
        for param in &function.params {
            self.ctx
                .add_variable(param.name.to_owned(), param.typ, false);
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
                self.ctx.add_variable(name.to_owned(), *typ, false);
                if value.is_none() {
                    return;
                }
                if let Some(value_typ) = self.apply_expression(value.as_deref().unwrap()) {
                    if &value_typ != typ {
                        self.issue(Error::new(
                            stmt.pos.clone(),
                            ErrorKind::TypeMismatch {
                                lhs: *typ,
                                rhs: value_typ,
                            },
                        ));
                    }
                }
            }
            StatementKind::Val { name, typ, value } => {
                self.ctx.add_variable(name.to_owned(), *typ, true);
                if value.is_none() {
                    return;
                }
                if let Some(value_typ) = self.apply_expression(value.as_deref().unwrap()) {
                    if &value_typ != typ {
                        self.issue(Error::new(
                            stmt.pos.clone(),
                            ErrorKind::TypeMismatch {
                                lhs: *typ,
                                rhs: value_typ,
                            },
                        ));
                    }
                }
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
                                    lhs: *ret_typ,
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

    fn apply_expression(&mut self, expr: &Expression) -> Option<Type> {
        use BinaryOperator::*;
        match &expr.kind {
            ExpressionKind::Integer { .. } => Some(Type::Int),
            ExpressionKind::Bool { .. } => Some(Type::Bool),
            ExpressionKind::Ident { name } => match self.ctx.find_variable(&name) {
                Some(var) => Some(var.typ),
                None => {
                    self.issue(Error::new(
                        expr.pos.clone(),
                        ErrorKind::NotDefinedVariable { name: name.into() },
                    ));
                    None
                }
            },
            ExpressionKind::UnaryOp { op, expr } => match self.apply_expression(&*expr)? {
                Type::Bool => Some(Type::Bool),
                typ => {
                    self.issue(Error::new(
                        expr.pos.clone(),
                        ErrorKind::UnaryOpErr { op: *op, expr: typ },
                    ));
                    None
                }
            },
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
        }
    }

    fn check_assign(&mut self, dst: &Expression, value: &Expression, pos: Pos) {
        let value_typ = self.apply_expression(&value);
        let (var, var_name) = match dst.kind {
            ExpressionKind::Ident { ref name } => {
                let var = self.ctx.find_variable(name);
                (var.cloned(), name)
            }
            _ => {
                self.issue(Error::new(dst.pos.clone(), ErrorKind::LvalueRequired));
                return;
            }
        };
        match (var, value_typ) {
            (Some(var), Some(value_typ)) => {
                if var.typ != value_typ {
                    self.issue(Error::new(
                        pos.clone(),
                        ErrorKind::TypeMismatch {
                            lhs: var.typ,
                            rhs: value_typ,
                        },
                    ));
                }
                if var.is_const {
                    self.issue(Error::new(
                        pos,
                        ErrorKind::AssignToConstant {
                            name: var_name.into(),
                        },
                    ));
                }
            }
            (None, _) => self.issue(Error::new(
                pos,
                ErrorKind::NotDefinedVariable {
                    name: var_name.into(),
                },
            )),
            _ => {}
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
                return Some(sig.ret_typ);
            }

            let param_types = sig.params.iter().map(|param| param.typ);

            for (arg_typ, param_typ) in arg_types.into_iter().zip(param_types) {
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

            Some(sig.ret_typ)
        };
        let ret_typ = do_check();

        issues.into_iter().for_each(|issue| self.issue(issue));
        ret_typ
    }

    fn issue(&mut self, err: Error) {
        self.issues.0.push(err);
    }
}
