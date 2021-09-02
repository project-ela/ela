use crate::common::{pos::Pos, symtab::NodeId, types::Type};

use super::Expression;

#[derive(Debug)]
pub struct Statement {
    pub kind: StatementKind,
    pub pos: Pos,
    pub id: NodeId,
}

impl Statement {
    pub fn new(kind: StatementKind, pos: Pos) -> Self {
        Self {
            kind,
            pos,
            id: NodeId::new(),
        }
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
        value: Option<Box<Expression>>,
    },
    Val {
        name: String,
        typ: Type,
        value: Option<Box<Expression>>,
    },
    Assign {
        dst: Box<Expression>,
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
