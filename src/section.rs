pub mod flags;
pub mod header;
pub mod typ;

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

#[derive(Eq, PartialEq)]
pub enum Flags {
    Write,
    Alloc,
    Execinstr,
    Merge,
    Strings,
    InfoLink,
    LinkOrder,
    OsNonconforming,
    Group,
    TLS,
    Compressed,
    Execlude,
}

#[derive(Eq, PartialEq)]
pub enum Type {
    Null,
    Progbits,
    Symtab,
    Strtab,
    Rela,
    Hash,
    Dynamic,
    Note,
    Nobits,
    Rel,
    Shlib,
    Dynsym,
    InitArray,
    FiniArray,
    Group,
    SymtabShndx,
    Unknown(u32),
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
