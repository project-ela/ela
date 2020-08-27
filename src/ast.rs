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

    Add {
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },
    Sub {
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },
    Mul {
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },
    Div {
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },

    Equal {
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },
    NotEqual {
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },

    Lt {
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },
    Lte {
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },
    Gt {
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },
    Gte {
        lhs: Box<AstExpression>,
        rhs: Box<AstExpression>,
    },
}
