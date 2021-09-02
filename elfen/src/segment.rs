pub mod flags;
pub mod typ;

use crate::*;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct ProgramHeader {
    pub typ: ElfWord,
    pub flags: ElfWord,
    pub offset: ElfOff,
    pub virt_addr: ElfAddr,
    pub phys_addr: ElfAddr,
    pub file_size: ElfXword,
    pub memory_size: ElfXword,
    pub alignment: ElfXword,
}

#[derive(Eq, PartialEq)]
pub enum Flags {
    X,
    W,
    R,
}

#[derive(Eq, PartialEq)]
pub enum Type {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Tls,
    Unknown(u32),
}

impl ProgramHeader {
    pub fn get_type(&self) -> Type {
        Type::from(self.typ)
    }

    pub fn set_type(&mut self, typ: Type) {
        self.typ = typ.into();
    }

    pub fn set_flags(&mut self, flag: Flags) {
        let bytes: u32 = flag.into();
        self.flags |= bytes;
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.typ.to_le_bytes());
        buf.extend(&self.flags.to_le_bytes());
        buf.extend(&self.offset.to_le_bytes());
        buf.extend(&self.virt_addr.to_le_bytes());
        buf.extend(&self.phys_addr.to_le_bytes());
        buf.extend(&self.file_size.to_le_bytes());
        buf.extend(&self.memory_size.to_le_bytes());
        buf.extend(&self.alignment.to_le_bytes());
    }
}
