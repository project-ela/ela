#[derive(Debug)]
pub enum Token {
    IntLiteral { value: u32 },

    Plus,
    Minus,

    EOF,
}
