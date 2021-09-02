use super::Type;

impl From<u32> for Type {
    fn from(bytes: u32) -> Self {
        match bytes {
            0 => Type::Null,
            1 => Type::Load,
            2 => Type::Dynamic,
            3 => Type::Interp,
            4 => Type::Note,
            5 => Type::Shlib,
            6 => Type::Phdr,
            7 => Type::Tls,
            x => Type::Unknown(x),
        }
    }
}

impl Into<u32> for Type {
    fn into(self) -> u32 {
        match self {
            Type::Null => 0,
            Type::Load => 1,
            Type::Dynamic => 2,
            Type::Interp => 3,
            Type::Note => 4,
            Type::Shlib => 5,
            Type::Phdr => 6,
            Type::Tls => 7,
            Type::Unknown(x) => x,
        }
    }
}
