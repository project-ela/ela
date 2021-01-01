use super::OSABI;

impl From<u8> for OSABI {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OSABI::OSABISysV,
            x => OSABI::Unknown(x),
        }
    }
}

impl Into<u8> for OSABI {
    fn into(self) -> u8 {
        match self {
            OSABI::OSABISysV => 0,
            OSABI::Unknown(x) => x,
        }
    }
}
