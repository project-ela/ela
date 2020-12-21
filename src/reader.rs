use header::ElfHeader;
use section::{ElfSectionHeader, Section};
use segment::ElfProgramHeader;

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

    fn read_header(bytes: &[u8]) -> ElfHeader {
        let (_, body, _) = unsafe { bytes.align_to::<ElfHeader>() };
        *&body[0]
    }

    fn read_section_headers(header: &ElfHeader, bytes: &[u8]) -> Vec<Section> {
        let mut sections = Vec::new();

        let hdr_num = header.section_header_num as usize;
        let hdr_off = header.section_header_offset as usize;
        let hdr_size = header.section_header_size as usize;

        for i in 0..hdr_num {
            // read section header
            let start_addr = hdr_off + hdr_size * i;
            let end_addr = start_addr + hdr_size;
            // スライスだと失敗する
            let header_bytes = bytes[start_addr..end_addr].to_vec();
            let (_, body, _) = unsafe { header_bytes.align_to::<ElfSectionHeader>() };
            let section_header = *&body[0];

            // read section data
            let start_addr = hdr_off as usize;
            let end_addr = start_addr + section_header.size as usize;
            let data = bytes[start_addr..end_addr].to_vec();

            // add section
            sections.push(Section {
                name: "".into(),
                header: section_header,
                data,
            });
        }

        // read section name
        let shstrtab_data = sections[header.string_table_index as usize].data.clone();
        for section in sections.iter_mut() {
            section.name = Self::get_name_from_strtab(&shstrtab_data, section.header.name as usize);
        }

        sections
    }

    fn read_program_headers(header: &ElfHeader, bytes: &[u8]) -> Vec<ElfProgramHeader> {
        let mut headers = Vec::new();

        let hdr_num = header.program_header_num as usize;
        let hdr_off = header.program_header_offset as usize;
        let hdr_size = header.program_header_size as usize;

        for i in 0..hdr_num {
            let start_addr = hdr_off + hdr_size * i;
            let end_addr = start_addr + hdr_size;
            let header_bytes = bytes[start_addr..end_addr].to_vec();
            let (_, body, _) = unsafe { header_bytes.align_to::<ElfProgramHeader>() };
            let program_header = *&body[0];

            headers.push(program_header);
        }

        headers
    }

    fn get_name_from_strtab(data: &[u8], index: usize) -> String {
        data[index..]
            .iter()
            .take_while(|&&v| v != 0)
            .map(|&v| v as char)
            .collect()
    }
}
