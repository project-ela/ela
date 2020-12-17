use super::register::Register;

#[derive(Debug, Clone)]
pub struct Memory {
    pub base: Register,
    pub disp: Option<Displacement>,
}

#[derive(Debug, Clone)]
pub enum Displacement {
    Disp8(i8),
    Disp32(i32),
}

impl Memory {
    pub fn new(base: Register, disp: Option<Displacement>) -> Self {
        Self { base, disp }
    }
}

impl Displacement {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Displacement::Disp8(value) => vec![*value as u8],
            Displacement::Disp32(value) => value.to_le_bytes().to_vec(),
        }
    }
}
