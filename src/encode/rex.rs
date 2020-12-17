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

    pub fn to_byte(&self) -> u8 {
        0b01000000
            | (self.w as u8) << 3
            | (self.r as u8) << 2
            | (self.x as u8) << 1
            | (self.b as u8)
    }
}
