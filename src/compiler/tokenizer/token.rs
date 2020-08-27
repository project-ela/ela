#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    IntLiteral { value: u32 },
    Ident { name: String },

    Func,
    Return,
    If,
    Else,

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
