use super::Class;

impl From<u8> for Class {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Class::ClassNone,
            1 => Class::Class32,
            2 => Class::Class64,
            x => Class::Unknown(x),
        }
    }
}

impl Into<u8> for Class {
    fn into(self) -> u8 {
        match self {
            Class::ClassNone => 0,
            Class::Class32 => 1,
            Class::Class64 => 2,
            Class::Unknown(x) => x,
        }
    }
}
