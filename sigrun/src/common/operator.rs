#[derive(Debug, Copy, Clone)]
pub enum UnaryOperator {
    Not,
    Addr,
    Load,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,

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

#[derive(Debug)]
pub enum BinOpType {
    Arithmetic,
    Comparison,
}

impl BinaryOperator {
    pub fn typ(&self) -> BinOpType {
        use self::BinaryOperator::*;

        match self {
            Equal | NotEqual | Lt | Lte | Gt | Gte => BinOpType::Comparison,
            Add | Sub | Mul | Div | Mod | And | Or | Xor => BinOpType::Arithmetic,
        }
    }
}
