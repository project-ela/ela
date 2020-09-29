use crate::{
    common::{operator::BinaryOperator, types::Type},
    frontend::parser::ast::*,
};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

struct SymbolPass {
    ctx: Context,
    issues: Vec<String>,
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
    variables: HashMap<String, Type>,
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

    fn add_variable(&mut self, name: String, typ: Type) {
        self.last_mut().unwrap().variables.insert(name, typ);
    }

    fn find_function(&self, name: &str) -> Option<&Type> {
        for ctx in self.iter().rev() {
            if ctx.functions.contains_key(name) {
                return ctx.functions.get(name);
            }
        }

        return None;
    }

    fn find_variable(&self, name: &str) -> Option<&Type> {
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

pub fn apply(program: &Program) -> Result<(), String> {
    let mut pass = SymbolPass::new();
    pass.apply(program)
}

impl SymbolPass {
    fn new() -> Self {
        Self {
            ctx: Context::new(),
            issues: Vec::new(),
        }
    }

    fn apply(&mut self, program: &Program) -> Result<(), String> {
        if program.functions.iter().all(|f| f.name != "main") {
            self.issue("there must be 'main' function");
        }

        for function in &program.functions {
            if function.name == "main" && function.ret_typ != Type::Int {
                self.issue("'main' function should return int value");
            }
            self.apply_function(&function);
        }

        if self.issues.is_empty() {
            Ok(())
        } else {
            Err(self.issues.join("\n"))
        }
    }

    fn apply_function(&mut self, funciton: &Function) {
        self.ctx
            .add_function(funciton.name.to_owned(), funciton.ret_typ);
        self.ctx.push_ctx();
        self.apply_statement(&funciton.body, &funciton.ret_typ);
        self.ctx.pop_ctx();
    }

    fn apply_statement(&mut self, stmt: &AstStatement, ret_typ: &Type) {
        match stmt {
            AstStatement::Block { stmts } => {
                self.ctx.push_ctx();
                for stmt in stmts {
                    self.apply_statement(stmt, ret_typ);
                }
                self.ctx.pop_ctx();
            }
            AstStatement::Declare { name, typ, value } => {
                if let Some(value_typ) = self.apply_expression(value) {
                    if &value_typ != typ {
                        self.issue(format!("type mismatch {} and {}", typ, value_typ));
                    }
                }
                self.ctx.add_variable(name.to_owned(), *typ);
            }
            AstStatement::Assign { name, value } => {
                let value_typ = self.apply_expression(value);
                let var_typ = self.ctx.find_variable(name);
                match (var_typ, value_typ) {
                    (Some(var_typ), Some(value_typ)) => {
                        if var_typ != &value_typ {
                            self.issue(format!("type mismatch {} and {}", var_typ, value_typ));
                        }
                    }
                    (None, _) => self.issue(format!("undefined variable: {}", name)),
                    _ => {}
                }
            }
            AstStatement::Return { value } => {
                if let Some(value) = value {
                    if let Some(value_typ) = self.apply_expression(value) {
                        if &value_typ != ret_typ {
                            self.issue(format!("type mismatch {} and {}", ret_typ, value_typ));
                        }
                    }
                }
            }
            AstStatement::If { cond, then, els } => {
                if self.apply_expression(cond) != Some(Type::Bool) {
                    self.issue("expression in if statement should be typed bool");
                }
                self.apply_statement(then, ret_typ);
                if let Some(els) = els {
                    self.apply_statement(els, ret_typ);
                }
            }
            AstStatement::While { cond, body } => {
                if self.apply_expression(cond) != Some(Type::Bool) {
                    self.issue("expression in while statement should be typed bool");
                }
                self.apply_statement(body, ret_typ);
            }
            AstStatement::Call { name } => {
                self.check_call(name);
            }
        }
    }

    fn apply_expression(&mut self, expr: &AstExpression) -> Option<Type> {
        use BinaryOperator::*;
        match expr {
            AstExpression::Integer { .. } => Some(Type::Int),
            AstExpression::Bool { .. } => Some(Type::Bool),
            AstExpression::Ident { name } => match self.ctx.find_variable(name) {
                Some(typ) => Some(*typ),
                None => {
                    self.issue(format!("undefined variable: {}", name));
                    None
                }
            },
            AstExpression::UnaryOp { op, expr } => match self.apply_expression(expr)? {
                Type::Bool => Some(Type::Bool),
                typ => {
                    self.issue(format!("cannot {:?} {:?}", op, typ));
                    None
                }
            },
            AstExpression::BinaryOp { op, lhs, rhs } => {
                let lhs_typ = self.apply_expression(lhs)?;
                let rhs_typ = self.apply_expression(rhs)?;
                if lhs_typ != rhs_typ {
                    self.issue(format!("mismatched types {:?} and {:?}", lhs_typ, rhs_typ));
                    return None;
                }
                match op {
                    Equal | NotEqual | Lt | Lte | Gt | Gte => Some(Type::Bool),
                    Add | Sub | Mul | Div | And | Or | Xor => match lhs_typ {
                        Type::Int => Some(Type::Int),
                        _ => {
                            self.issue(format!("cannot {:?} {:?} and {:?}", op, lhs_typ, rhs_typ));
                            None
                        }
                    },
                }
            }
            AstExpression::Call { name } => self.check_call(name),
        }
    }

    fn check_call(&mut self, name: &str) -> Option<Type> {
        match self.ctx.find_function(name) {
            Some(typ) => Some(*typ),
            None => {
                self.issue(format!("undefined function: {}", name));
                None
            }
        }
    }

    fn issue<T: Into<String>>(&mut self, msg: T) {
        self.issues.push(msg.into());
    }
}
