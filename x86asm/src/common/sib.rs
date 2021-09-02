#[derive(Default)]
pub struct Sib {
    pub scale: u8,
    pub index: u8,
    pub base: u8,
}

impl Sib {
    pub fn new(scale: u8, index: u8, base: u8) -> Self {
        Self { scale, index, base }
    }

    pub fn from_byte(code: u8) -> Self {
        Self::new(
            (code & 0b11000000) >> 6,
            (code & 0b00111000) >> 3,
            code & 0b00000111,
        )
    }

    pub fn to_byte(&self) -> u8 {
        (self.scale << 6) | (self.index << 3) | self.base
    }
}
