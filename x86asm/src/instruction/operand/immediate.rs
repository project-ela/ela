#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Immediate {
    Imm8(i8),
    Imm32(i32),
}

impl Immediate {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Immediate::Imm8(value) => vec![*value as u8],
            Immediate::Imm32(value) => value.to_le_bytes().to_vec(),
        }
    }
}
