use std::mem::size_of;

use section::SectionData;
use segment::ProgramHeader;

use crate::header::Header;
use crate::section::{Section, SectionHeader};
use crate::*;

pub struct Elf {
    pub header: Header,
    pub sections: Vec<Section>,
    pub segments: Vec<ProgramHeader>,
}

impl Default for Elf {
    fn default() -> Self {
        Self {
            header: Header::new(),
            sections: Vec::new(),
            segments: Vec::new(),
        }
    }
}

impl Elf {
    pub fn add_section(&mut self, name: &str, header: SectionHeader, data: SectionData) {
        self.sections.push(Section {
            name: name.to_string(),
            header,
            data,
        });
    }

    pub fn add_segment(&mut self, header: ProgramHeader) {
        self.segments.push(header);
    }

    pub fn find_section(&self, name: &str) -> Option<usize> {
        self.sections
            .iter()
            .enumerate()
            .filter(|(_, section)| section.name == name)
            .next()
            .map(|(i, _)| i)
    }

    pub fn finalize(&mut self) {
        self.header.section_header_num = self.sections.len() as u16;
        self.header.string_table_index = self.find_section(".shstrtab").unwrap() as u16;

        let mut offset = size_of::<Header>() as u64;
        for section in self.sections.iter_mut() {
            let section_size = section.data.len() as u64;
            section.header.offset = offset;
            section.header.size = section_size;
            offset += section_size;
        }
        self.header.section_header_offset = offset;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();

        self.header.write_to(&mut result);

        for segment in &self.segments {
            segment.write_to(&mut result);
        }

        let mut section_indices = self
            .sections
            .iter()
            .enumerate()
            .map(|(i, v)| (i, v.header.offset))
            .collect::<Vec<(usize, u64)>>();
        section_indices.sort_by_key(|(_, offset)| *offset);

        for (index, _) in section_indices {
            let section = self.sections.get(index).unwrap();
            add_padding(&mut result, section.header.offset as usize);
            section.data.write_to(&mut result);
        }

        add_padding(&mut result, self.header.section_header_offset as usize);
        for section in &self.sections {
            section.header.write_to(&mut result);
        }
        result
    }
}

fn add_padding(v: &mut Vec<u8>, offset: usize) {
    if offset < v.len() {
        return;
    }
    let padding_len = offset - v.len();
    v.extend(&vec![0; padding_len as usize]);
}
