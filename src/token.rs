#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    IntLiteral { value: u32 },

    Plus,
    Minus,
    Asterisk,
    Slash,

    LParen,
    RParen,

    EOF,
}
