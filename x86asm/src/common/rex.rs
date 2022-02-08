pub struct Rex {
    pub w: bool,
    pub r: bool,
    pub x: bool,
    pub b: bool,
}

impl Rex {
    pub fn new(w: bool, r: bool, x: bool, b: bool) -> Self {
        Self { w, r, x, b }
    }

    pub fn from_byte(code: u8) -> Self {
        Self::new(
            (code & 0b00001000) == 0b00001000,
            (code & 0b00000100) == 0b00000100,
            (code & 0b00000010) == 0b00000010,
            (code & 0b00000001) == 0b00000001,
        )
    }

    pub fn to_byte(&self) -> u8 {
        0b01000000
            | (self.w as u8) << 3
            | (self.r as u8) << 2
            | (self.x as u8) << 1
            | (self.b as u8)
    }
}
