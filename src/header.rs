pub mod class;
pub mod data;
pub mod machine;
pub mod osabi;
pub mod typ;

use std::mem::size_of;

use crate::{section::SectionHeader, *};

#[repr(C)]
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct Header {
    pub ident: ElfIdent,
    pub filetype: ElfHalf,
    pub machine: ElfHalf,
    pub version: ElfWord,
    pub entrypoint: ElfAddr,
    pub program_header_offset: ElfOff,
    pub section_header_offset: ElfOff,
    pub flags: ElfWord,
    pub elf_header_size: ElfHalf,
    pub program_header_size: ElfHalf,
    pub program_header_num: ElfHalf,
    pub section_header_size: ElfHalf,
    pub section_header_num: ElfHalf,
    pub string_table_index: ElfHalf,
}

#[derive(Eq, PartialEq)]
pub enum Class {
    ClassNone,
    Class32,
    Class64,
    Unknown(u8),
}

#[derive(Eq, PartialEq)]
pub enum Data {
    DataNone,
    Data2LSB,
    Data2MSB,
    Unknown(u8),
}

#[derive(Eq, PartialEq)]
pub enum Machine {
    None,
    X86,
    X86_64,
    Unknown(u16),
}

#[derive(Eq, PartialEq)]
pub enum OSABI {
    OSABISysV,
    Unknown(u8),
}

#[derive(Eq, PartialEq)]
pub enum Type {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Unknown(u16),
}

impl Header {
    pub fn new() -> Self {
        let mut hdr: Self = Default::default();
        hdr.ident = 0x7f454c46 << (12 * 8);
        hdr.ident |= 0x1 << (9 * 8); // version
        hdr.version = 0x1;
        hdr.elf_header_size = size_of::<Header>() as u16;
        hdr.section_header_size = size_of::<SectionHeader>() as u16;
        hdr
    }

    pub fn get_class(&self) -> Class {
        let byte = (self.ident >> (11 * 8)) as u8;
        Class::from(byte)
    }

    pub fn set_class(&mut self, class: Class) {
        let byte: u8 = class.into();
        self.ident |= (byte as u128) << (11 * 8);
    }

    pub fn get_data(&self) -> Data {
        let byte = (self.ident >> (10 * 8)) as u8;
        Data::from(byte)
    }

    pub fn set_data(&mut self, data: Data) {
        let byte: u8 = data.into();
        self.ident |= (byte as u128) << (10 * 8);
    }

    pub fn get_osabi(&self) -> OSABI {
        let byte = (self.ident >> (8 * 8)) as u8;
        OSABI::from(byte)
    }

    pub fn set_osabi(&mut self, osabi: OSABI) {
        let byte: u8 = osabi.into();
        self.ident |= (byte as u128) << (8 * 8);
    }

    pub fn get_filetype(&self) -> Type {
        Type::from(self.filetype)
    }
    pub fn set_filetype(&mut self, typ: Type) {
        self.filetype = typ.into();
    }

    pub fn get_machine(&self) -> Machine {
        Machine::from(self.machine)
    }

    pub fn set_machine(&mut self, machine: Machine) {
        self.machine = machine.into();
    }

    pub fn write_to(&self, buf: &mut Vec<u8>) {
        buf.extend(&self.ident.to_be_bytes());
        buf.extend(&self.filetype.to_le_bytes());
        buf.extend(&self.machine.to_le_bytes());
        buf.extend(&self.version.to_le_bytes());
        buf.extend(&self.entrypoint.to_le_bytes());
        buf.extend(&self.program_header_offset.to_le_bytes());
        buf.extend(&self.section_header_offset.to_le_bytes());
        buf.extend(&self.flags.to_le_bytes());
        buf.extend(&self.elf_header_size.to_le_bytes());
        buf.extend(&self.program_header_size.to_le_bytes());
        buf.extend(&self.program_header_num.to_le_bytes());
        buf.extend(&self.section_header_size.to_le_bytes());
        buf.extend(&self.section_header_num.to_le_bytes());
        buf.extend(&self.string_table_index.to_le_bytes());
    }
}
