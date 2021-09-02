#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Offset {
    Off8(i8),
    Off32(i32),
}

impl Offset {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Offset::Off8(value) => value.to_le_bytes().to_vec(),
            Offset::Off32(value) => value.to_le_bytes().to_vec(),
        }
    }
}
