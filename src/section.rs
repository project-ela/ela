use std::mem::size_of;

use rel::Rela;
use strtab::Strtab;
use symbol::Symbol;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub name: String,
    pub header: SectionHeader,
    pub data: SectionData,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SectionData {
    None,
    Raw(Vec<u8>),
    Rela(Vec<Rela>),
    Strtab(Strtab),
    Symbols(Vec<Symbol>),
}

impl SectionData {
    pub fn len(&self) -> usize {
        match self {
            SectionData::None => 0,
            SectionData::Raw(data) => data.len(),
            SectionData::Rela(relas) => size_of::<Rela>() * relas.len(),
            SectionData::Strtab(strtab) => strtab.data.len(),
            SectionData::Symbols(symbols) => size_of::<Symbol>() * symbols.len(),
        }
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) {
        match self {
            SectionData::None => {}
            SectionData::Raw(data) => buf.extend(data),
            SectionData::Rela(relas) => {
                for rela in relas {
                    rela.write_to(buf);
                }
            }
            SectionData::Strtab(strtab) => buf.extend(&strtab.data),
            SectionData::Symbols(symbols) => {
                for sym in symbols {
                    sym.write_to(buf);
                }
            }
        }
    }

    pub fn as_raw(&self) -> Option<&Vec<u8>> {
        if let SectionData::Raw(data) = self {
            return Some(data);
        }
        None
    }

    pub fn as_raw_mut(&mut self) -> Option<&mut Vec<u8>> {
        if let SectionData::Raw(data) = self {
            return Some(data);
        }
        None
    }

    pub fn as_rela(&self) -> Option<&Vec<Rela>> {
        if let SectionData::Rela(rela) = self {
            return Some(rela);
        }
        None
    }

    pub fn as_rela_mut(&mut self) -> Option<&mut Vec<Rela>> {
        if let SectionData::Rela(rela) = self {
            return Some(rela);
        }
        None
    }

    pub fn as_strtab(&self) -> Option<&Strtab> {
        if let SectionData::Strtab(strtab) = self {
            return Some(strtab);
        }
        None
    }

    pub fn as_strtab_mut(&mut self) -> Option<&mut Strtab> {
        if let SectionData::Strtab(strtab) = self {
            return Some(strtab);
        }
        None
    }

    pub fn as_symbols(&self) -> Option<&Vec<Symbol>> {
        if let SectionData::Symbols(symbols) = self {
            return Some(symbols);
        }
        None
    }

    pub fn as_symbols_mut(&mut self) -> Option<&mut Vec<Symbol>> {
        if let SectionData::Symbols(symbols) = self {
            return Some(symbols);
        }
        None
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct SectionHeader {
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

#[repr(u64)]
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

impl SectionHeader {
    pub fn set_name(&mut self, name: ElfWord) {
        self.name = name;
    }

    pub fn set_type(&mut self, typ: Type) {
        self.section_type = typ as ElfWord;
    }

    pub fn set_flags(&mut self, flag: Flags) {
        self.flags |= flag as ElfXword;
    }

    pub fn set_addr(&mut self, addr: ElfAddr) {
        self.addr = addr;
    }

    pub fn set_offset(&mut self, offset: ElfOff) {
        self.offset = offset;
    }

    pub fn set_size(&mut self, size: ElfXword) {
        self.size = size;
    }

    pub fn set_link(&mut self, link: ElfWord) {
        self.link = link;
    }

    pub fn set_info(&mut self, info: ElfWord) {
        self.info = info;
    }

    pub fn set_align(&mut self, align: ElfXword) {
        self.alignment = align;
    }

    pub fn set_entry_size(&mut self, ent_size: ElfXword) {
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
