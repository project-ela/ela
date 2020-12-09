use std::mem::size_of;

use crate::header::ElfHeader;
use crate::section::{ElfSectionHeader, Section};
use crate::symbol::ElfSymbol;
use crate::*;

#[allow(dead_code)]
const SYM_ENTRY_SIZE_32: ElfXword = 0x10;
#[allow(dead_code)]
const SYM_ENTRY_SIZE_64: ElfXword = 0x18;

pub struct Elf {
    pub header: ElfHeader,
    pub sections: Vec<Section>,
}

impl Default for Elf {
    fn default() -> Self {
        Self {
            header: ElfHeader::new(),
            sections: Vec::new(),
        }
    }
}

impl Elf {
    pub fn new() -> Self {
        let mut elf = Self::default();

        elf.add_null_section();
        elf.add_shstrtab();
        elf.add_symtab();

        elf
    }

    pub fn add_section(&mut self, name: &str, header: ElfSectionHeader, data: Vec<u8>) {
        let mut header = header;

        let strtab = self.find_section_mut(".shstrtab").unwrap();
        header.name = strtab.data.len() as u32;
        strtab.data.extend(name.as_bytes());
        strtab.data.push(0);

        self.sections.push(Section {
            name: name.to_string(),
            header,
            data,
        });
        self.header.section_header_num += 1;
    }

    pub fn find_section_mut(&mut self, name: &str) -> Option<&mut Section> {
        self.sections
            .iter_mut()
            .filter(|section| section.name == name)
            .next()
    }

    pub fn add_symbol(&mut self, name: String, symbol: ElfSymbol) {
        let mut symbol = symbol;

        let strtab = self.find_section_mut(".strtab").unwrap();
        symbol.name = strtab.data.len() as u32;
        strtab.data.extend(name.as_bytes());
        strtab.data.push(0);

        let symtab = self.find_section_mut(".symtab").unwrap();
        symbol.write_to(&mut symtab.data);
        symtab.header.set_info(symtab.header.info + 1);
    }

    pub fn update_elf_header(&mut self) {
        let mut data_length = 0;
        data_length += size_of::<ElfHeader>();
        for section in self.sections.as_mut_slice() {
            section.header.offset = data_length as ElfOff;
            section.header.size = section.data.len() as ElfXword;
            data_length += section.data.len();
        }
        self.header.section_header_offset = data_length as ElfOff;
    }

    fn add_symtab(&mut self) {
        let mut symtab_hdr = ElfSectionHeader::default();
        symtab_hdr.set_type(section::Type::Symtab);
        symtab_hdr.set_link(self.sections.len() as u32 + 1);
        symtab_hdr.set_entry_size(SYM_ENTRY_SIZE_64);
        symtab_hdr.set_align(8);
        self.add_section(".symtab", symtab_hdr, Vec::new());

        let mut strtab_hdr = ElfSectionHeader::default();
        strtab_hdr.set_type(section::Type::Strtab);
        strtab_hdr.set_align(1);
        self.add_section(".strtab", strtab_hdr, Vec::new());
    }

    fn add_shstrtab(&mut self) {
        let mut shstrtab_hdr = section::ElfSectionHeader::default();
        shstrtab_hdr.set_type(section::Type::Strtab);
        shstrtab_hdr.set_align(1);

        let mut data = Vec::new();
        data.push(0);
        shstrtab_hdr.name = data.len() as u32;
        data.extend(b".shstrtab\0");

        self.sections.push(Section {
            name: ".shstrtab".to_string(),
            header: shstrtab_hdr,
            data,
        });

        self.header.section_header_num += 1;
        self.header.string_table_index = self.sections.len() as ElfHalf - 1;
    }

    fn add_null_section(&mut self) {
        let header = ElfSectionHeader::default();
        self.sections.push(Section {
            name: "\0".to_string(),
            header,
            data: Vec::new(),
        });
        self.header.section_header_num += 1;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.header.write_to(&mut result);
        for section in &self.sections {
            result.extend(&section.data);
        }
        for section in &self.sections {
            section.header.write_to(&mut result);
        }
        result
    }
}
