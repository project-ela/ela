use super::Type;

impl From<u32> for Type {
    fn from(bytes: u32) -> Self {
        match bytes {
            0 => Type::Null,
            1 => Type::Progbits,
            2 => Type::Symtab,
            3 => Type::Strtab,
            4 => Type::Rela,
            5 => Type::Hash,
            6 => Type::Dynamic,
            7 => Type::Note,
            8 => Type::Nobits,
            9 => Type::Rel,
            10 => Type::Shlib,
            11 => Type::Dynsym,
            14 => Type::InitArray,
            15 => Type::FiniArray,
            17 => Type::Group,
            18 => Type::SymtabShndx,
            x => Type::Unknown(x),
        }
    }
}

impl Into<u32> for Type {
    fn into(self) -> u32 {
        match self {
            Type::Null => 0,
            Type::Progbits => 1,
            Type::Symtab => 2,
            Type::Strtab => 3,
            Type::Rela => 4,
            Type::Hash => 5,
            Type::Dynamic => 6,
            Type::Note => 7,
            Type::Nobits => 8,
            Type::Rel => 9,
            Type::Shlib => 10,
            Type::Dynsym => 11,
            Type::InitArray => 14,
            Type::FiniArray => 15,
            Type::Group => 17,
            Type::SymtabShndx => 18,
            Type::Unknown(x) => x,
        }
    }
}
