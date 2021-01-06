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
