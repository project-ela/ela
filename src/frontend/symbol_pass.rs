pub mod error;
use crate::{
    common::{
        error::{Error, Errors},
        operator::{BinaryOperator, UnaryOperator},
        pos::Pos,
        types::Type,
    },
    frontend::ast::*,
};
use std::collections::HashMap;

use self::error::PassError;

struct SymbolPass {
    ctx: Context,
    issues: Errors,

    cur_ret_typ: Option<Type>,
}

struct Context(Vec<ContextData>);

#[derive(Default)]
struct ContextData {
    functions: HashMap<String, FunctionSig>,
    variables: HashMap<String, Variable>,
}

struct FunctionSig {
    params: Vec<Parameter>,
    ret_typ: Type,
}

struct Variable {
    typ: Type,
    is_const: bool,
}

impl Context {
    fn new() -> Self {
        let mut ctx = Self(Vec::new());
        ctx.push();
        ctx
    }

    fn add_function(&mut self, name: String, params: &[Parameter], ret_typ: Type) {
        let mut new_params = Vec::new();
        for param in params {
            new_params.push(param.clone());
        }

        self.0.last_mut().unwrap().functions.insert(
            name,
            FunctionSig {
                params: new_params,
                ret_typ,
            },
        );
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

pub fn apply(program: &mut Program) -> Result<(), Errors> {
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
            cur_ret_typ: None,
        }
    }

    fn apply(&mut self, program: &mut Program) {
        if program.functions.iter().all(|f| f.name != "main") {
            self.issue(Error::new(Pos::default(), PassError::MainNotFound));
        }

        for global_def in &program.global_defs {
            self.ctx.add_variable(
                global_def.name.clone(),
                global_def.typ.clone(),
                global_def.is_const,
            );
        }

        for function in program.functions.iter_mut() {
            self.apply_function(function);
        }
    }

    fn apply_function(&mut self, function: &mut Function) {
        self.ctx.add_function(
            function.name.to_owned(),
            &function.params,
            function.ret_typ.clone(),
        );

        if function.name == "main" && function.ret_typ != Type::Int {
            self.issue(Error::new(
                function.pos.clone(),
                PassError::MainShouldReturnInt,
            ));
        }

        self.ctx.push();
        for param in &function.params {
            self.ctx
                .add_variable(param.name.clone(), param.typ.clone(), false);
        }
        self.cur_ret_typ = Some(function.ret_typ.clone());

        if function.body.is_some() {
            self.apply_statement(function.body.as_mut().unwrap());
        }

        self.ctx.pop();
    }

    fn apply_statement(&mut self, stmt: &mut Statement) {
        match &mut stmt.kind {
            StatementKind::Block { ref mut stmts } => {
                self.ctx.push();
                for stmt in stmts {
                    self.apply_statement(stmt);
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
                self.apply_assign(&mut *dst, &mut *value, stmt.pos.clone())
            }
            StatementKind::Return { value } => {
                if let Some(value) = value {
                    if let Some(value_typ) = self.apply_expression(&mut *value) {
                        if !value_typ.is_same(self.cur_ret_typ.as_ref().unwrap()) {
                            self.issue(Error::new(
                                stmt.pos.clone(),
                                PassError::TypeMismatch(
                                    self.cur_ret_typ.clone().unwrap(),
                                    value_typ,
                                ),
                            ));
                        }
                    }
                }
            }
            StatementKind::If { cond, then, els } => {
                match self.apply_expression(&mut *cond) {
                    Some(Type::Bool) | None => {}
                    Some(x) => self.issue(Error::new(
                        stmt.pos.clone(),
                        PassError::TypeMismatch(x, Type::Bool),
                    )),
                }
                self.apply_statement(&mut *then);
                if let Some(els) = els {
                    self.apply_statement(&mut *els);
                }
            }
            StatementKind::While { cond, body } => {
                match self.apply_expression(&mut *cond) {
                    Some(Type::Bool) | None => {}
                    Some(x) => self.issue(Error::new(
                        stmt.pos.clone(),
                        PassError::TypeMismatch(x, Type::Bool),
                    )),
                }
                self.apply_statement(&mut *body);
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
        value: &mut Option<Box<Expression>>,
        pos: &Pos,
        is_const: bool,
    ) {
        if self.ctx.find_variable_scope(name).is_some() {
            self.issue(Error::new(
                pos.clone(),
                PassError::RedefinitionOf(name.clone()),
            ));
        }

        self.ctx.add_variable(name.clone(), typ.clone(), is_const);

        let value_typ = value
            .as_mut()
            .and_then(|value| self.apply_expression(value));
        if let Some(value_typ) = value_typ {
            if !value_typ.is_same(typ) {
                self.issue(Error::new(
                    pos.clone(),
                    PassError::TypeMismatch(typ.clone(), value_typ),
                ));
            }
        }
    }

    fn apply_expression(&mut self, expr: &mut Expression) -> Option<Type> {
        use BinaryOperator::*;
        use UnaryOperator::*;
        let typ = match &mut expr.kind {
            ExpressionKind::Char { .. } => Some(Type::Byte),
            ExpressionKind::Integer { .. } => Some(Type::Int),
            ExpressionKind::String { .. } => Some(Type::Byte.pointer_to()),
            ExpressionKind::Bool { .. } => Some(Type::Bool),
            ExpressionKind::Ident { name } => match self.ctx.find_variable(&name) {
                Some(var) => Some(var.typ.clone()),
                None => {
                    self.issue(Error::new(
                        expr.pos.clone(),
                        PassError::NotDefinedVariable(name.clone()),
                    ));
                    None
                }
            },
            ExpressionKind::UnaryOp { op, expr } => {
                let expr_typ = self.apply_expression(&mut *expr)?;
                match op {
                    Not => match expr_typ {
                        Type::Bool => Some(Type::Bool),
                        typ => {
                            self.issue(Error::new(
                                expr.pos.clone(),
                                PassError::UnaryOpErr(*op, typ),
                            ));
                            None
                        }
                    },
                    Addr => match expr.kind {
                        ExpressionKind::Ident { .. } => Some(expr_typ.pointer_to()),
                        _ => {
                            self.issue(Error::new(expr.pos.clone(), PassError::LvalueRequired));
                            None
                        }
                    },
                    Load => match expr_typ {
                        Type::Pointer { pointer_to } => Some(*pointer_to),
                        x => {
                            self.issue(Error::new(expr.pos.clone(), PassError::CannotLoad(x)));
                            None
                        }
                    },
                }
            }
            ExpressionKind::BinaryOp { op, lhs, rhs } => {
                let lhs_typ = self.apply_expression(&mut *lhs)?;
                let rhs_typ = self.apply_expression(&mut *rhs)?;
                if !lhs_typ.is_same(&rhs_typ) {
                    self.issue(Error::new(
                        expr.pos.clone(),
                        PassError::TypeMismatch(lhs_typ, rhs_typ),
                    ));
                    return None;
                }
                match op {
                    Equal | NotEqual | Lt | Lte | Gt | Gte => Some(Type::Bool),
                    Add | Sub | Mul | Div | Mod | And | Or | Xor => match lhs_typ {
                        Type::Byte => Some(Type::Byte),
                        Type::Int => Some(Type::Int),
                        _ => {
                            self.issue(Error::new(
                                expr.pos.clone(),
                                PassError::BinaryOpErr(*op, lhs_typ, rhs_typ),
                            ));
                            None
                        }
                    },
                }
            }
            ExpressionKind::Call { name, args } => self.check_call(&name, args, expr.pos.clone()),
            ExpressionKind::Index {
                ref mut lhs,
                ref mut index,
            } => {
                let lhs_typ = self.apply_expression(lhs)?;
                let index_typ = self.apply_expression(index)?;
                match lhs_typ {
                    Type::Array { elm_type: typ, .. } | Type::Pointer { pointer_to: typ } => {
                        if index_typ != Type::Int {
                            self.issue(Error::new(
                                expr.pos.clone(),
                                PassError::TypeMismatch(index_typ, Type::Int),
                            ));
                        }
                        Some(*typ)
                    }
                    _ => {
                        self.issue(Error::new(
                            expr.pos.clone(),
                            PassError::CannotIndex(lhs_typ),
                        ));
                        None
                    }
                }
            }
        };

        expr.typ = typ.clone();
        typ
    }

    fn apply_assign(&mut self, dst: &mut Expression, value: &mut Expression, pos: Pos) {
        let dst_typ = self.apply_expression(dst);
        let value_typ = self.apply_expression(value);
        match (dst_typ, value_typ) {
            (Some(dst_typ), Some(value_typ)) => {
                if !dst_typ.is_same(&value_typ) {
                    self.issue(Error::new(
                        pos.clone(),
                        PassError::TypeMismatch(dst_typ, value_typ),
                    ));
                }
            }
            _ => return,
        }

        match &dst.kind {
            ExpressionKind::Ident { name } => {
                let var = self.ctx.find_variable(name).unwrap();
                if var.is_const {
                    self.issue(Error::new(pos, PassError::AssignToConstant(name.into())));
                }
            }
            ExpressionKind::Index { .. } => {}
            ExpressionKind::UnaryOp {
                op: UnaryOperator::Load,
                ..
            } => {}
            _ => self.issue(Error::new(pos, PassError::LvalueRequired)),
        }
    }

    // TODO refactor
    fn check_call(&mut self, name: &str, args: &mut [Expression], pos: Pos) -> Option<Type> {
        let mut issues = Vec::new();

        let arg_types: Vec<Option<Type>> = args
            .iter_mut()
            .map(|mut arg| self.apply_expression(&mut arg))
            .collect();

        let do_check = || {
            let sig = if let Some(sig) = self.ctx.find_function(name) {
                sig
            } else {
                self.issue(Error::new(pos, PassError::NotDefinedFunction(name.into())));
                return None;
            };

            if args.len() != sig.params.len() {
                issues.push(Error::new(
                    pos.clone(),
                    PassError::FunctionArgNum(name.to_string(), sig.params.len(), args.len()),
                ));
                return Some(sig.ret_typ.clone());
            }

            let param_types = sig.params.iter().map(|param| param.typ.clone());

            for (arg_typ, param_typ) in arg_types.into_iter().zip(param_types) {
                if let Some(arg_typ) = arg_typ {
                    if !arg_typ.is_same(&param_typ) {
                        issues.push(Error::new(
                            pos.clone(),
                            PassError::TypeMismatch(arg_typ, param_typ),
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
