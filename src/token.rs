#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    IntLiteral { value: u32 },
    Ident { name: String },

    Func,
    Return,
    If,

    Equal,
    NotEqual,

    Lt,
    Lte,
    Gt,
    Gte,

    Plus,
    Minus,
    Asterisk,
    Slash,

    LParen,
    RParen,
    LBrace,
    RBrace,

    EOF,
}
