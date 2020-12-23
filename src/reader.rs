use header::Header;
use rel::Rela;
use section::{Section, SectionData, SectionHeader};
use segment::ProgramHeader;
use strtab::Strtab;
use symbol::Symbol;

use crate::elf::*;
use crate::*;
use std::fs;

impl Elf {
    pub fn read_from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let header = Self::read_header(&bytes);
        let sections = Self::read_section_headers(&header, &bytes);
        let segments = Self::read_program_headers(&header, &bytes);

        Elf {
            header,
            sections,
            segments,
        }
    }

    fn read_header(bytes: &[u8]) -> Header {
        let (_, body, _) = unsafe { bytes.align_to::<Header>() };
        let mut header = body[0];
        header.ident = header.ident.to_be();
        header
    }

    fn read_section_headers(header: &Header, bytes: &[u8]) -> Vec<Section> {
        let mut sections = Vec::new();

        let hdr_num = header.section_header_num as usize;
        let hdr_off = header.section_header_offset as usize;
        let hdr_size = header.section_header_size as usize;

        for i in 0..hdr_num {
            // read section header
            let start_addr = hdr_off + hdr_size * i;
            let end_addr = start_addr + hdr_size;
            let header_bytes = bytes[start_addr..end_addr].to_vec();
            let (_, body, _) = unsafe { header_bytes.align_to::<SectionHeader>() };
            let section_header = body[0];

            // read section data
            let start_addr = section_header.offset as usize;
            let end_addr = start_addr + section_header.size as usize;
            let data = bytes[start_addr..end_addr].to_vec();

            // add section
            sections.push(Section {
                name: "".into(),
                header: section_header,
                data: Self::read_section_data(&section_header, data),
            });
        }

        // read section name
        let shstrtab_data = sections[header.string_table_index as usize]
            .data
            .as_strtab()
            .unwrap()
            .clone();

        for section in sections.iter_mut() {
            section.name = shstrtab_data.get(section.header.name as usize);
        }

        sections
    }

    fn read_section_data(header: &SectionHeader, data: Vec<u8>) -> SectionData {
        match header.section_type {
            x if x == section::Type::Null as u32 => SectionData::None,
            x if x == section::Type::Rela as u32 => {
                let relas = Self::read_relas(header, data);
                SectionData::Rela(relas)
            }
            x if x == section::Type::Strtab as u32 => SectionData::Strtab(Strtab::new(data)),
            x if x == section::Type::Symtab as u32 => {
                let symbols = Self::read_symbols(header, data);
                SectionData::Symbols(symbols)
            }
            _ => SectionData::Raw(data),
        }
    }

    fn read_symbols(header: &SectionHeader, data: Vec<u8>) -> Vec<Symbol> {
        let mut symbols = Vec::new();
        let symbol_size = header.entry_size as usize;
        let symbol_num = data.len() / symbol_size;

        for i in 0..symbol_num {
            let start_addr = symbol_size * i;
            let end_addr = start_addr + symbol_size;
            let symol_bytes = data[start_addr..end_addr].to_vec();
            let (_, body, _) = unsafe { symol_bytes.align_to::<Symbol>() };
            let symbol = body[0];
            symbols.push(symbol);
        }
        symbols
    }

    fn read_relas(header: &SectionHeader, data: Vec<u8>) -> Vec<Rela> {
        let mut relas = Vec::new();
        let rela_size = header.entry_size as usize;
        let rela_num = data.len() / rela_size;

        for i in 0..rela_num {
            let start_addr = rela_size * i;
            let end_addr = start_addr + rela_size;
            let symol_bytes = data[start_addr..end_addr].to_vec();
            let (_, body, _) = unsafe { symol_bytes.align_to::<Rela>() };
            let rela = body[0];
            relas.push(rela);
        }
        relas
    }

    fn read_program_headers(header: &Header, bytes: &[u8]) -> Vec<ProgramHeader> {
        let mut headers = Vec::new();

        let hdr_num = header.program_header_num as usize;
        let hdr_off = header.program_header_offset as usize;
        let hdr_size = header.program_header_size as usize;

        for i in 0..hdr_num {
            let start_addr = hdr_off + hdr_size * i;
            let end_addr = start_addr + hdr_size;
            let header_bytes = bytes[start_addr..end_addr].to_vec();
            let (_, body, _) = unsafe { header_bytes.align_to::<ProgramHeader>() };
            let program_header = body[0];

            headers.push(program_header);
        }

        headers
    }
}
