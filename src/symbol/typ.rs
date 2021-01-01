use super::Type;

impl From<u8> for Type {
    fn from(bytes: u8) -> Self {
        match bytes {
            0 => Type::Notype,
            1 => Type::Object,
            2 => Type::Func,
            3 => Type::Section,
            4 => Type::File,
            5 => Type::Common,
            6 => Type::TLS,
            x => Type::Unknown(x),
        }
    }
}

impl Into<u8> for Type {
    fn into(self) -> u8 {
        match self {
            Type::Notype => 0,
            Type::Object => 1,
            Type::Func => 2,
            Type::Section => 3,
            Type::File => 4,
            Type::Common => 5,
            Type::TLS => 6,
            Type::Unknown(x) => x,
        }
    }
}
