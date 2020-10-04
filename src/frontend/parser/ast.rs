use crate::common::{
    operator::{BinaryOperator, UnaryOperator},
    pos::Pos,
    types::Type,
};

#[derive(Debug, Default)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub ret_typ: Type,
    pub body: Statement,
    pub pos: Pos,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug)]
pub struct Statement {
    pub kind: StatementKind,
    pub pos: Pos,
}

impl Statement {
    pub fn new(kind: StatementKind, pos: Pos) -> Self {
        Self { kind, pos }
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
        args: Vec<Expression>,
    },
}

#[derive(Debug)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub pos: Pos,
}

impl Expression {
    pub fn new(kind: ExpressionKind, pos: Pos) -> Self {
        Self { kind, pos }
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
        args: Vec<Expression>,
    },
}
