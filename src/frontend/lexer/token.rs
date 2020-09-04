#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    IntLiteral { value: u32 },
    Ident { name: String },

    Func,
    Var,
    Return,
    If,
    Else,
    False,
    True,

    Assign,

    And,
    Or,
    Xor,

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

    Colon,

    EOF,
}
