use crate::{ElfAddr, ElfOff, ElfWord, ElfXword};

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ElfProgramHeader {
    pub typ: ElfWord,
    pub flags: ElfWord,
    pub offset: ElfOff,
    pub virt_addr: ElfAddr,
    pub phys_addr: ElfAddr,
    pub file_size: ElfXword,
    pub memory_size: ElfXword,
    pub alignment: ElfXword,
}

pub enum Type {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interp = 3,
    Note = 4,
    Shlib = 5,
    Phdr = 6,
    Tls = 7,
}

pub enum Flags {
    X = 1 << 0,
    W = 1 << 1,
    R = 1 << 2,
}

impl ElfProgramHeader {
    pub fn set_type(&mut self, typ: Type) {
        self.typ = typ as ElfWord;
    }

    pub fn set_flags(&mut self, flag: Flags) {
        self.flags |= flag as ElfWord;
    }

    pub fn set_offset(&mut self, offset: ElfOff) {
        self.offset = offset;
    }

    pub fn set_virtual_address(&mut self, virt_addr: ElfAddr) {
        self.virt_addr = virt_addr;
    }

    pub fn set_physical_address(&mut self, phys_addr: ElfAddr) {
        self.phys_addr = phys_addr;
    }

    pub fn set_file_size(&mut self, file_size: ElfXword) {
        self.file_size = file_size;
    }

    pub fn set_memory_size(&mut self, memory_size: ElfXword) {
        self.memory_size = memory_size;
    }

    pub fn set_alignment(&mut self, alignment: ElfXword) {
        self.alignment = alignment;
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
