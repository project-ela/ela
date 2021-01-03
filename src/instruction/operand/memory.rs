use super::register::Register;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    pub base: Option<Register>,
    pub disp: Option<Displacement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Displacement {
    Disp8(i8),
    Disp32(i32),
}

impl Memory {
    pub fn new(base: Register, disp: Option<Displacement>) -> Self {
        Self {
            base: Some(base),
            disp,
        }
    }

    pub fn new_disp(disp: Displacement) -> Self {
        Self {
            base: None,
            disp: Some(disp),
        }
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
