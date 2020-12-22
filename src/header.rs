use crate::*;

#[repr(C)]
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub struct ElfHeader {
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

pub enum Class {
    ClassNone = 0,
    Class32 = 1,
    Class64 = 2,
}

pub enum Data {
    DataNone = 0,
    Data2LSB = 1,
    Data2MSB = 2,
}

pub enum OSABI {
    OSABISysV = 0,
}

pub enum Type {
    None = 0,
    Rel = 1,
    Exec = 2,
    Dyn = 3,
    Core = 4,
}

pub enum Machine {
    None = 0,
    X86 = 3,
    X86_64 = 62,
}

#[allow(dead_code)]
const ELF_HDR_SIZE_32: ElfHalf = 52;
#[allow(dead_code)]
const SECTION_HDR_SIZE_32: ElfHalf = 40;

#[allow(dead_code)]
const ELF_HDR_SIZE_64: ElfHalf = 64;
#[allow(dead_code)]
const SECTION_HDR_SIZE_64: ElfHalf = 64;

impl ElfHeader {
    pub fn new() -> Self {
        let mut hdr: Self = Default::default();
        hdr.ident = 0x7f454c46 << (12 * 8);
        hdr.ident |= 0x1 << (9 * 8); // version
        hdr.version = 0x1;
        hdr.elf_header_size = ELF_HDR_SIZE_64;
        hdr.section_header_size = SECTION_HDR_SIZE_64;
        hdr
    }

    pub fn set_class(&mut self, class: Class) {
        self.ident |= (class as u128) << (11 * 8);
    }

    pub fn set_data(&mut self, data: Data) {
        self.ident |= (data as u128) << (10 * 8);
    }

    pub fn set_osabi(&mut self, osabi: OSABI) {
        self.ident |= (osabi as u128) << (8 * 8);
    }

    pub fn set_filetype(&mut self, typ: Type) {
        self.filetype = typ as ElfHalf;
    }

    pub fn set_machine(&mut self, machine: Machine) {
        self.machine = machine as ElfHalf;
    }

    pub fn set_entrypoint(&mut self, addr: ElfAddr) {
        self.entrypoint = addr;
    }

    pub fn set_program_header_offset(&mut self, offset: ElfOff) {
        self.program_header_offset = offset;
    }

    pub fn set_section_header_offset(&mut self, offset: ElfOff) {
        self.section_header_offset = offset;
    }

    pub fn set_program_header_size(&mut self, size: ElfHalf) {
        self.program_header_size = size;
    }

    pub fn set_program_header_num(&mut self, num: ElfHalf) {
        self.program_header_num = num;
    }

    pub fn set_section_header_size(&mut self, size: ElfHalf) {
        self.section_header_size = size;
    }

    pub fn set_section_header_num(&mut self, num: ElfHalf) {
        self.section_header_num = num;
    }

    pub fn set_string_header_num(&mut self, index: ElfHalf) {
        self.string_table_index = index;
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
