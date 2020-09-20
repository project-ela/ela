use crate::common::{
    operator::{BinaryOperator, UnaryOperator},
    types::Type,
};

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub ret_typ: Type,
    pub body: AstStatement,
}

#[derive(Debug)]
pub enum AstStatement {
    Block {
        stmts: Vec<AstStatement>,
    },

    Declare {
        name: String,
        typ: Type,
        value: Box<AstExpression>,
    },
    Assign {
        name: String,
        value: Box<AstExpression>,
    },
    Return {
        value: Option<Box<AstExpression>>,
    },
    If {
        cond: Box<AstExpression>,
        then: Box<AstStatement>,
        els: Option<Box<AstStatement>>,
    },
    While {
        cond: Box<AstExpression>,
        body: Box<AstStatement>,
    },
    // use this if return type is void
    Call {
        name: String,
    },
}

#[derive(Debug)]
pub enum AstExpression {
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
        expr: Box<AstExpression>,
    },
    BinaryOp {
        op: BinaryOperator,
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },
    // use this if return type isn't void
    Call {
        name: String,
    },
}
