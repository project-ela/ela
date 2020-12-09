pub mod elf;
pub mod header;
pub mod section;
pub mod symbol;

type ElfHalf = u16;
type ElfWord = u32;
type ElfXword = u64;
type ElfAddr = u64;
type ElfOff = u64;
type ElfSection = u16;
type ElfIdent = u128;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
