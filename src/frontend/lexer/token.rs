#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Token {
    IntLiteral { value: i32 },
    Ident { name: String },

    Comment { content: String },

    Func,
    Var,
    Val,
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
