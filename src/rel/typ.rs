use super::Type;

impl From<u64> for Type {
    fn from(bytes: u64) -> Self {
        match bytes {
            0 => Type::None,
            4 => Type::Plt32,
            x => Type::Unknown(x),
        }
    }
}

impl Into<u64> for Type {
    fn into(self) -> u64 {
        match self {
            Type::None => 0,
            Type::Plt32 => 4,
            Type::Unknown(x) => x,
        }
    }
}
