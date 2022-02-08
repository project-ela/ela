use super::IndexType;

impl From<u16> for IndexType {
    fn from(bytes: u16) -> Self {
        match bytes {
            0 => IndexType::Undef,
            0xfff1 => IndexType::Abs,
            0xfff2 => IndexType::Common,
            x => IndexType::Index(x),
        }
    }
}

impl Into<u16> for IndexType {
    fn into(self) -> u16 {
        match self {
            IndexType::Undef => 0,
            IndexType::Abs => 0xfff1,
            IndexType::Common => 0xfff2,
            IndexType::Index(x) => x,
        }
    }
}
