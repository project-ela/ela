pub mod elf;
pub mod header;
pub mod reader;
pub mod rel;
pub mod section;
pub mod segment;
pub mod strtab;
pub mod symbol;

type ElfHalf = u16;
type ElfWord = u32;
type ElfXword = u64;
type ElfSxword = i64;
type ElfAddr = u64;
type ElfOff = u64;
type ElfSection = u16;
type ElfIdent = u128;
