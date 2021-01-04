use x86asm::instruction::{mnemonic::Mnemonic, operand::register::Register};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
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
