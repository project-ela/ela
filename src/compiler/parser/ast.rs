use std::collections::HashMap;

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
    pub ctx: Context,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub variables: HashMap<String, Variable>,
    pub cur_offset: u32,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            cur_offset: 0,
        }
    }

    pub fn add_variable(&mut self, name: &String) {
        self.cur_offset += 4; // TODO
        self.variables.insert(
            name.clone(),
            Variable {
                name: name.clone(),
                offset: self.cur_offset,
            },
        );
    }

    pub fn find_variable(&self, name: &String) -> Option<&Variable> {
        self.variables.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub offset: u32,
}

#[derive(Debug)]
pub enum AstStatement {
    Block {
        stmts: Vec<AstStatement>,
    },

    Declare {
        name: String,
        value: Box<AstExpression>,
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
    Ident {
        name: String,
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
