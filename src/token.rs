use crate::instruction::{Mnemonic, Register};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    Integer(u32),
    Ident(String),
    Symbol(Symbol),
    Mnemonic(Mnemonic),
    Register(Register),
    EOF,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Symbol {
    Comma,
    Colon,
}
