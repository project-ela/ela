use crate::common::pos::Pos;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Pos,
}

impl Token {
    pub fn new(kind: TokenKind, pos: Pos) -> Self {
        Self { kind, pos }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenKind {
    CharLiteral { value: char },
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
    LBracket,
    RBracket,
    Comma,

    Colon,

    EOF,
}
