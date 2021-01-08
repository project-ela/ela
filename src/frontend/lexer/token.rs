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
    Char(char),
    Integer(i32),
    Ident(String),
    Comment(String),
    Keyword(Keyword),
    Symbol(Symbol),
    EOF,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Keyword {
    Func,
    Var,
    Val,
    Return,
    If,
    Else,
    False,
    True,
    While,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Symbol {
    Assign,
    PlusAssign,
    MinusAssign,
    AsteriskAssign,
    SlashAssign,

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
    Percent,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,

    Colon,
}
