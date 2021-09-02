use super::Flags;

impl Flags {
    pub fn contained_in(self, flags: u64) -> bool {
        flags & Into::<u64>::into(self) != 0
    }
}

impl Into<u64> for Flags {
    fn into(self) -> u64 {
        let n = match self {
            Flags::Write => 0,
            Flags::Alloc => 1,
            Flags::Execinstr => 2,
            Flags::Merge => 4,
            Flags::Strings => 5,
            Flags::InfoLink => 6,
            Flags::LinkOrder => 7,
            Flags::OsNonconforming => 8,
            Flags::Group => 9,
            Flags::TLS => 10,
            Flags::Compressed => 11,
            Flags::Execlude => 31,
        };
        1 << n
    }
}
