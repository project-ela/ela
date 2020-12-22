use section::SectionData;
use segment::ElfProgramHeader;

use crate::header::ElfHeader;
use crate::section::{ElfSectionHeader, Section};
use crate::*;

pub struct Elf {
    pub header: ElfHeader,
    pub sections: Vec<Section>,
    pub segments: Vec<ElfProgramHeader>,
}

impl Default for Elf {
    fn default() -> Self {
        Self {
            header: ElfHeader::new(),
            sections: Vec::new(),
            segments: Vec::new(),
        }
    }
}

impl Elf {
    pub fn add_section(&mut self, name: &str, header: ElfSectionHeader, data: SectionData) {
        self.sections.push(Section {
            name: name.to_string(),
            header,
            data,
        });
    }

    pub fn add_segment(&mut self, header: ElfProgramHeader) {
        self.segments.push(header);
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        self.header.write_to(&mut result);
        for segment in &self.segments {
            segment.write_to(&mut result);
        }
        for section in &self.sections {
            section.data.write_to(&mut result);
        }
        for section in &self.sections {
            section.header.write_to(&mut result);
        }
        result
    }
}
