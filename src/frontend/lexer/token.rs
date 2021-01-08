use x86asm::instruction::{mnemonic::Mnemonic, operand::register::Register};

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

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TokenKind {
    Integer(u32),
    Ident(String),
    Symbol(Symbol),
    Keyword(Keyword),
    Mnemonic(Mnemonic),
    Register(Register),
    Comment(String),
    EOF,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Symbol {
    Comma,
    Colon,
    LBracket,
    RBracket,
    Plus,
    Minus,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Keyword {
    Byte,
    Ptr,
}
