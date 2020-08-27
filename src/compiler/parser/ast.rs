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
    pub body: AstStatement,
}

#[derive(Debug)]
pub enum AstStatement {
    Block {
        stmts: Vec<AstStatement>,
    },

    Return {
        value: Box<AstExpression>,
    },
    If {
        cond: Box<AstExpression>,
        then: Box<AstStatement>,
        els: Option<Box<AstStatement>>,
    },
}

#[derive(Debug)]
pub enum AstExpression {
    Integer {
        value: u32,
    },

    BinaryOp {
        op: Operator,
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,

    And,
    Or,
    Xor,

    Equal,
    NotEqual,

    Lt,
    Lte,
    Gt,
    Gte,
}
