use header::Header;
use rel::Rela;
use section::{Section, SectionData};
use segment::ProgramHeader;
use strtab::Strtab;
use symbol::Symbol;
use tse::Tse;

use crate::{elf::Elf, section::SectionHeader, *};
use std::fs;

impl Elf {
    pub fn read_from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let header = Self::read_header(&bytes);
        let sections = Self::read_sections(&header, &bytes);
        let segments = Self::read_segments(&header, &bytes);

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

    fn read_sections(header: &Header, bytes: &[u8]) -> Vec<Section> {
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
                data: Self::read_section_data(&section_header, &data),
            });
        }

        // read section name
        let strtab = sections[header.string_table_index as usize]
            .data
            .as_strtab()
            .unwrap()
            .clone();

        // resolve section name
        for section in sections.iter_mut() {
            section.name = strtab.get(section.header.name as usize);

            if section.name == ".tse_info" {
                section.data = SectionData::Tse(Self::read_entries::<Tse>(
                    section.data.as_raw().unwrap(),
                    section.header.entry_size as usize,
                ));
            }
        }

        sections
    }

    fn read_section_data(header: &SectionHeader, data: &[u8]) -> SectionData {
        match header.get_type() {
            section::Type::Null => SectionData::None,
            section::Type::Rela => {
                let relas = Self::read_entries::<Rela>(data, header.entry_size as usize);
                SectionData::Rela(relas)
            }
            section::Type::Strtab => SectionData::Strtab(Strtab::new(data.to_vec())),
            section::Type::Symtab => {
                let symbols = Self::read_entries::<Symbol>(data, header.entry_size as usize);
                SectionData::Symbols(symbols)
            }
            _ => SectionData::Raw(data.to_vec()),
        }
    }

    fn read_segments(header: &Header, bytes: &[u8]) -> Vec<ProgramHeader> {
        Self::read_entries_num::<ProgramHeader>(
            bytes,
            header.program_header_offset as usize,
            header.program_header_size as usize,
            header.program_header_num as usize,
        )
    }

    fn read_entries<T: Copy>(data: &[u8], size: usize) -> Vec<T> {
        Self::read_entries_num(data, 0, size, data.len() / size)
    }

    fn read_entries_num<T: Copy>(data: &[u8], offset: usize, size: usize, num: usize) -> Vec<T> {
        let mut entries = Vec::new();
        for i in 0..num {
            let start_addr = offset + size * i;
            let end_addr = start_addr + size;
            let symol_bytes = data[start_addr..end_addr].to_vec();
            let (_, body, _) = unsafe { symol_bytes.align_to::<T>() };
            entries.push(body[0]);
        }
        entries
    }
}
