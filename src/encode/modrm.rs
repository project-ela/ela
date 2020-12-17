pub struct ModRM {
    pub modval: u8,
    pub reg: u8,
    pub rm: u8,
}

impl ModRM {
    pub fn new(modval: u8, reg: u8, rm: u8) -> Self {
        Self { modval, reg, rm }
    }

    pub fn to_byte(&self) -> u8 {
        (self.modval << 6) | (self.reg << 3) | self.rm
    }
}
