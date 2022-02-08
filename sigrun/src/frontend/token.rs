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
    EOF,

    Char(char),
    Integer(i32),
    String(String),
    Ident(String),
    Comment(String),

    Keyword(Keyword),
    Symbol(Symbol),
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
    And,
    Assign,
    Asterisk,
    AsteriskAssign,
    Colon,
    Comma,
    Equal,
    Gt,
    Gte,
    LBrace,
    LBracket,
    LParen,
    Lt,
    Lte,
    Minus,
    MinusAssign,
    Not,
    NotEqual,
    Or,
    Percent,
    Plus,
    PlusAssign,
    RBrace,
    RBracket,
    RParen,
    Slash,
    SlashAssign,
    Xor,
}
