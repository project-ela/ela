pub mod elf_header;
pub mod section_header;
pub mod symbol;

use crate::elf::elf_header::ElfHeader;
use crate::elf::section_header::ElfSectionHeader;
use crate::elf::symbol::ElfSymbol;

type ElfHalf = u16;
type ElfWord = u32;
type ElfXword = u64;
type ElfAddr = u64;
type ElfOff = u64;
type ElfSection = u16;
type ElfIdent = u128;

pub struct Elf {
    pub elf_header: ElfHeader,
    pub sections: Vec<Section>,
    pub section_names: Vec<u8>,
    pub symbols: Vec<ElfSymbol>,
    pub symbol_names: Vec<u8>,
}

pub struct Section {
    pub header: ElfSectionHeader,
    pub data: Vec<u8>,
}

impl Elf {
    pub fn new() -> Self {
        Self {
            elf_header: ElfHeader::new(),
            sections: Vec::new(),
            section_names: Vec::new(),
            symbols: Vec::new(),
            symbol_names: Vec::new(),
        }
    }

    pub fn add_section(&mut self, name: String, header: ElfSectionHeader, data: Vec<u8>) {
        let name_index = self.section_names.len();
        self.section_names.extend(name.as_bytes());
        self.section_names.push(0x0);

        let mut header = header.clone(); // TODO FIXME
        header.name = name_index as u32;
        self.sections.push(Section { header, data });

        self.elf_header.section_header_num += 1;
    }

    pub fn add_symbol(&mut self, name: String, symbol: ElfSymbol) {
        let name_index = self.symbol_names.len();
        self.symbol_names.extend(name.as_bytes());
        self.symbol_names.push(0x0);

        let mut symbol = symbol.clone(); // TODO FIXME
        symbol.name = name_index as u32;

        self.symbols.push(symbol);
    }

    pub fn update_elf_header(&mut self) {
        self.add_symtab();
        self.add_shstrtab();

        let mut data_length = 0;
        data_length += 64;
        for section in self.sections.as_mut_slice() {
            section.header.offset = data_length as u64;
            section.header.size = section.data.len() as u64;
            data_length += section.data.len();
        }
        self.elf_header.section_header_offset = data_length as u64;
    }

    fn add_symtab(&mut self) {
        let mut symtab_hdr = ElfSectionHeader::new();
        symtab_hdr.set_type(section_header::Type::Symtab);
        symtab_hdr.set_link(self.sections.len() as u32 + 1);
        symtab_hdr.set_info(self.symbols.len() as u32 - 1);
        symtab_hdr.set_entry_size(0x18);
        symtab_hdr.set_align(8);
        let mut symbol_data = Vec::new();
        for symbol in &self.symbols {
            symbol.write_to(&mut symbol_data);
        }
        self.add_section(".symtab".to_string(), symtab_hdr, symbol_data);

        let mut strtab_hdr = ElfSectionHeader::new();
        strtab_hdr.set_type(section_header::Type::Strtab);
        strtab_hdr.set_align(1);
        self.add_section(".strtab".to_string(), strtab_hdr, self.symbol_names.clone());
    }

    fn add_shstrtab(&mut self) {
        let mut shstrtab_hdr = section_header::ElfSectionHeader::new();
        shstrtab_hdr.name = self.section_names.len() as u32;
        shstrtab_hdr.set_type(section_header::Type::Strtab);
        shstrtab_hdr.set_align(1);
        self.section_names.extend(".shstrtab".as_bytes());
        self.section_names.push(0x0);

        self.sections.push(Section {
            header: shstrtab_hdr,
            data: self.section_names.clone(),
        });

        self.elf_header.section_header_num += 1;
        self.elf_header.string_table_index = self.sections.len() as u16 - 1;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.elf_header.write_to(&mut result);
        for section in &self.sections {
            result.extend(&section.data);
        }
        for section in &self.sections {
            section.header.write_to(&mut result);
        }
        result
    }
}
