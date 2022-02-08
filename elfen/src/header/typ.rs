use super::Type;

impl From<u16> for Type {
    fn from(bytes: u16) -> Self {
        match bytes {
            0 => Type::None,
            1 => Type::Rel,
            2 => Type::Exec,
            3 => Type::Dyn,
            4 => Type::Core,
            x => Type::Unknown(x),
        }
    }
}

impl Into<u16> for Type {
    fn into(self) -> u16 {
        match self {
            Type::None => 0,
            Type::Rel => 1,
            Type::Exec => 2,
            Type::Dyn => 3,
            Type::Core => 4,
            Type::Unknown(x) => x,
        }
    }
}
