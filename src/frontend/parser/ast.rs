use crate::common::{
    operator::{BinaryOperator, UnaryOperator},
    types::Type,
};

#[derive(Debug, Default)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub ret_typ: Type,
    pub body: Statement,
}

#[derive(Debug)]
pub struct Statement {
    pub kind: StatementKind,
}

impl Statement {
    pub fn new(kind: StatementKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug)]
pub enum StatementKind {
    Block {
        stmts: Vec<Statement>,
    },

    Var {
        name: String,
        typ: Type,
        value: Box<Expression>,
    },
    Val {
        name: String,
        typ: Type,
        value: Box<Expression>,
    },
    Assign {
        name: String,
        value: Box<Expression>,
    },
    Return {
        value: Option<Box<Expression>>,
    },
    If {
        cond: Box<Expression>,
        then: Box<Statement>,
        els: Option<Box<Statement>>,
    },
    While {
        cond: Box<Expression>,
        body: Box<Statement>,
    },
    // use this if return type is void
    Call {
        name: String,
    },
}

#[derive(Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
}

impl Expression {
    pub fn new(kind: ExpressionKind) -> Self {
        Self { kind }
    }
}

#[derive(Debug)]
pub enum ExpressionKind {
    Integer {
        value: i32,
    },
    Bool {
        value: bool,
    },
    Ident {
        name: String,
    },

    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    BinaryOp {
        op: BinaryOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    // use this if return type isn't void
    Call {
        name: String,
    },
}
