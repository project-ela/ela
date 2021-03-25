use crate::common::{
    operator::{BinaryOperator, UnaryOperator},
    pos::Pos,
    symtab::NodeId,
};

#[derive(Debug, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub pos: Pos,
    pub id: NodeId,
}

impl Expression {
    pub fn new(kind: ExpressionKind, pos: Pos) -> Self {
        Self {
            kind,
            pos,
            id: NodeId::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Char {
        value: char,
    },
    Integer {
        value: i32,
    },
    String {
        value: String,
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
    Index {
        lhs: Box<Expression>,
        index: Box<Expression>,
    },
}
