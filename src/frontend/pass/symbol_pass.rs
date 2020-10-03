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
use std::ops::{Deref, DerefMut};

struct SymbolPass {
    ctx: Context,
    issues: Errors,
}

struct Context(Vec<ContextData>);

impl Deref for Context {
    type Target = Vec<ContextData>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
struct ContextData {
    functions: HashMap<String, Type>,
    variables: HashMap<String, Variable>,
}

#[derive(Clone)]
struct Variable {
    typ: Type,
    is_const: bool,
}

impl Context {
    fn new() -> Self {
        let mut ctx = Self(Vec::new());
        ctx.push_ctx();
        ctx
    }

    fn add_function(&mut self, name: String, ret_typ: Type) {
        self.last_mut().unwrap().functions.insert(name, ret_typ);
    }

    fn add_variable(&mut self, name: String, typ: Type, is_const: bool) {
        self.last_mut()
            .unwrap()
            .variables
            .insert(name, Variable { typ, is_const });
    }

    fn find_function(&self, name: &str) -> Option<&Type> {
        for ctx in self.iter().rev() {
            if ctx.functions.contains_key(name) {
                return ctx.functions.get(name);
            }
        }

        return None;
    }

    fn find_variable(&self, name: &str) -> Option<&Variable> {
        for ctx in self.iter().rev() {
            if ctx.variables.contains_key(name) {
                return ctx.variables.get(name);
            }
        }

        return None;
    }

    fn push_ctx(&mut self) {
        self.push(ContextData::default());
    }

    fn pop_ctx(&mut self) {
        self.pop();
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

impl SymbolPass {
    fn new() -> Self {
        Self {
            ctx: Context::new(),
            issues: Errors::default(),
        }
    }

    fn apply(&mut self, program: &Program) {
        if program.functions.iter().all(|f| f.name != "main") {
            self.issue(Error::new(Pos::default(), ErrorKind::MainNotFound));
        }

        for function in &program.functions {
            if function.name == "main" && function.ret_typ != Type::Int {
                self.issue(Error::new(Pos::default(), ErrorKind::MainShouldReturnInt));
            }
            self.apply_function(&function);
        }
    }

    fn apply_function(&mut self, funciton: &Function) {
        self.ctx
            .add_function(funciton.name.to_owned(), funciton.ret_typ);
        self.ctx.push_ctx();
        for param in &funciton.params {
            self.ctx
                .add_variable(param.name.to_owned(), param.typ, false);
        }
        self.apply_statement(&funciton.body, &funciton.ret_typ);
        self.ctx.pop_ctx();
    }

    fn apply_statement(&mut self, stmt: &Statement, ret_typ: &Type) {
        match &stmt.kind {
            StatementKind::Block { stmts } => {
                self.ctx.push_ctx();
                for stmt in stmts {
                    self.apply_statement(&stmt, ret_typ);
                }
                self.ctx.pop_ctx();
            }
            StatementKind::Var { name, typ, value } => {
                if let Some(value_typ) = self.apply_expression(&*value) {
                    if &value_typ != typ {
                        self.issue(Error::new(
                            Pos::default(),
                            ErrorKind::TypeMismatch {
                                lhs: *typ,
                                rhs: value_typ,
                            },
                        ));
                    }
                }
                self.ctx.add_variable(name.to_owned(), *typ, false);
            }
            StatementKind::Val { name, typ, value } => {
                if let Some(value_typ) = self.apply_expression(&*value) {
                    if &value_typ != typ {
                        self.issue(Error::new(
                            Pos::default(),
                            ErrorKind::TypeMismatch {
                                lhs: *typ,
                                rhs: value_typ,
                            },
                        ));
                    }
                }
                self.ctx.add_variable(name.to_owned(), *typ, true);
            }
            StatementKind::Assign { name, value } => {
                let value_typ = self.apply_expression(&*value);
                let var = self.ctx.find_variable(&name).cloned();
                match (var, value_typ) {
                    (Some(var), Some(value_typ)) => {
                        if var.typ != value_typ {
                            self.issue(Error::new(
                                Pos::default(),
                                ErrorKind::TypeMismatch {
                                    lhs: var.typ,
                                    rhs: value_typ,
                                },
                            ));
                        }
                        if var.is_const {
                            self.issue(Error::new(
                                Pos::default(),
                                ErrorKind::AssignToConstant { name: name.into() },
                            ));
                        }
                    }
                    (None, _) => self.issue(Error::new(
                        Pos::default(),
                        ErrorKind::NotDefinedVariable { name: name.into() },
                    )),
                    _ => {}
                }
            }
            StatementKind::Return { value } => {
                if let Some(value) = value {
                    if let Some(value_typ) = self.apply_expression(&*value) {
                        if &value_typ != ret_typ {
                            self.issue(Error::new(
                                Pos::default(),
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
                        Pos::default(),
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
                        Pos::default(),
                        ErrorKind::TypeMismatch {
                            lhs: x,
                            rhs: Type::Bool,
                        },
                    )),
                }
                self.apply_statement(&*body, ret_typ);
            }
            StatementKind::Call { name } => {
                self.check_call(&*name);
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
                        Pos::default(),
                        ErrorKind::NotDefinedVariable { name: name.into() },
                    ));
                    None
                }
            },
            ExpressionKind::UnaryOp { op, expr } => match self.apply_expression(&*expr)? {
                Type::Bool => Some(Type::Bool),
                typ => {
                    self.issue(Error::new(
                        Pos::default(),
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
                        Pos::default(),
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
                                Pos::default(),
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
            ExpressionKind::Call { name } => self.check_call(&name),
        }
    }

    fn check_call(&mut self, name: &str) -> Option<Type> {
        match self.ctx.find_function(name) {
            Some(typ) => Some(*typ),
            None => {
                self.issue(Error::new(
                    Pos::default(),
                    ErrorKind::NotDefinedFunction { name: name.into() },
                ));
                None
            }
        }
    }

    fn issue(&mut self, err: Error) {
        self.issues.0.push(err);
    }
}
