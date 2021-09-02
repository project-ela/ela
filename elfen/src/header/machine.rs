use super::Machine;

impl From<u16> for Machine {
    fn from(bytes: u16) -> Self {
        match bytes {
            0 => Machine::None,
            3 => Machine::X86,
            62 => Machine::X86_64,
            x => Machine::Unknown(x),
        }
    }
}

impl Into<u16> for Machine {
    fn into(self) -> u16 {
        match self {
            Machine::None => 0,
            Machine::X86 => 3,
            Machine::X86_64 => 62,
            Machine::Unknown(x) => x,
        }
    }
}
