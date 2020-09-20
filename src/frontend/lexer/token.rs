#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    IntLiteral { value: i32 },
    Ident { name: String },

    Func,
    Var,
    Return,
    If,
    Else,
    False,
    True,
    While,

    Assign,

    And,
    Or,
    Xor,
    Not,

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
