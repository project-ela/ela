use super::Flags;

impl Flags {
    pub fn contained_in(self, flags: u32) -> bool {
        flags & Into::<u32>::into(self) != 0
    }
}

impl Into<u32> for Flags {
    fn into(self) -> u32 {
        let n = match self {
            Flags::X => 0,
            Flags::W => 1,
            Flags::R => 2,
        };
        1 << n
    }
}
