use crate::{
    common::{operator::Operator, types::Type},
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

    fn find_function(&self, name: &String) -> Option<&Type> {
        for ctx in self.iter().rev() {
            if ctx.functions.contains_key(name) {
                return ctx.functions.get(name);
            }
        }

        return None;
    }

    fn find_variable(&self, name: &String) -> Option<&Type> {
        for ctx in self.iter().rev() {
            if ctx.variables.contains_key(name) {
                return ctx.variables.get(name);
            }
        }

        return None;
    }

    fn push_ctx(&mut self) {
        self.push(ContextData::new());
    }

    fn pop_ctx(&mut self) {
        self.pop();
    }
}

impl ContextData {
    fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
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
        for function in &program.functions {
            self.apply_function(&function);
        }

        if self.issues.is_empty() {
            Ok(())
        } else {
            Err(self.issues.join("\n"))
        }
    }

    fn apply_function(&mut self, funciton: &Function) {
        self.ctx.add_function(funciton.name.to_owned(), Type::Int);
        self.ctx.push_ctx();
        self.apply_statement(&funciton.body);
        self.ctx.pop_ctx();
    }

    fn apply_statement(&mut self, stmt: &AstStatement) {
        match stmt {
            AstStatement::Block { stmts } => {
                self.ctx.push_ctx();
                for stmt in stmts {
                    self.apply_statement(stmt);
                }
                self.ctx.pop_ctx();
            }
            AstStatement::Declare { name, typ, value } => {
                self.apply_expression(value);
                self.ctx.add_variable(name.to_owned(), typ.clone());
            }
            AstStatement::Assign { name, value } => {
                if self.ctx.find_variable(name).is_none() {
                    self.issue(format!("undefined variable: {}", name));
                }
                self.apply_expression(value);
            }
            AstStatement::Return { value } => {
                self.apply_expression(value);
            }
            AstStatement::If { cond, then, els } => {
                if self.apply_expression(cond) != Some(Type::Bool) {
                    self.issue("expression in if statement should be typed bool".to_string());
                }
                self.apply_statement(then);
                if let Some(els) = els {
                    self.apply_statement(els);
                }
            }
        }
    }

    fn apply_expression(&mut self, expr: &AstExpression) -> Option<Type> {
        use Operator::*;
        match expr {
            AstExpression::Integer { .. } => Some(Type::Int),
            AstExpression::Bool { .. } => Some(Type::Bool),
            AstExpression::Ident { name } => match self.ctx.find_variable(name) {
                Some(typ) => Some(typ.clone()),
                None => {
                    self.issue(format!("undefined variable: {}", name));
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
                Some(match op {
                    Add | Sub | Mul | Div | And | Or | Xor => Type::Int,
                    Equal | NotEqual | Lt | Lte | Gt | Gte => Type::Bool,
                })
            }
        }
    }

    fn issue(&mut self, msg: String) {
        self.issues.push(msg);
    }
}
