use crate::elf::*;

#[derive(Default, Copy, Clone)]
pub struct ElfSectionHeader {
    pub name: ElfWord,
    pub section_type: ElfWord,
    pub flags: ElfXword,
    pub addr: ElfAddr,
    pub offset: ElfOff,
    pub size: ElfXword,
    pub link: ElfWord,
    pub info: ElfWord,
    pub alignment: ElfXword,
    pub entry_size: ElfXword,
}

pub enum Type {
    Null = 0,
    Progbits = 1,
    Symtab = 2,
    Strtab = 3,
    Rela = 4,
    Hash = 5,
    Dynamic = 6,
    Note = 7,
    Nobits = 8,
    Rel = 9,
    Shlib = 10,
    Dynsym = 11,
    InitArray = 14,
    FiniArray = 15,
    Group = 17,
    SymtabShndx = 18,
}

pub enum Flags {
    Write = 1 << 0,
    Alloc = 1 << 1,
    Execinstr = 1 << 2,
    Merge = 1 << 4,
    Strings = 1 << 5,
    InfoLink = 1 << 6,
    LinkOrder = 1 << 7,
    OsNonconforming = 1 << 8,
    Group = 1 << 9,
    TLS = 1 << 10,
    Compressed = 1 << 11,
    Execlude = 1 << 31,
}

impl ElfSectionHeader {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_name(&mut self, name: u32) {
        self.name = name;
    }

    pub fn set_type(&mut self, typ: Type) {
        self.section_type = typ as u32;
    }

    pub fn set_flags(&mut self, flag: Flags) {
        self.flags |= flag as u64;
    }

    pub fn set_addr(&mut self, addr: u64) {
        self.addr = addr;
    }

    pub fn set_offset(&mut self, offset: u64) {
        self.offset = offset;
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_link(&mut self, link: u32) {
        self.link = link;
    }

    pub fn set_info(&mut self, info: u32) {
        self.info = info;
    }

    pub fn set_align(&mut self, align: u64) {
        self.alignment = align;
    }

    pub fn set_entry_size(&mut self, ent_size: u64) {
        self.entry_size = ent_size;
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.name.to_le_bytes());
        buf.extend(&self.section_type.to_le_bytes());
        buf.extend(&self.flags.to_le_bytes());
        buf.extend(&self.addr.to_le_bytes());
        buf.extend(&self.offset.to_le_bytes());
        buf.extend(&self.size.to_le_bytes());
        buf.extend(&self.link.to_le_bytes());
        buf.extend(&self.info.to_le_bytes());
        buf.extend(&self.alignment.to_le_bytes());
        buf.extend(&self.entry_size.to_le_bytes());
    }
}
