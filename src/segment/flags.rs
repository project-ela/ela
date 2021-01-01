use super::Flags;

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
