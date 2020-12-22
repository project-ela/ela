use elfen::{
    elf::Elf,
    header::{self, ElfHeader},
    section::{self, ElfSectionHeader, Section, SectionData},
    segment::{self, ElfProgramHeader},
    strtab::Strtab,
    symbol::ElfSymbol,
};
use std::fs;

fn ret_o_header() -> ElfHeader {
    ElfHeader {
        ident: 0x7f454c46020101000000000000000000,
        filetype: header::Type::Rel as u16,
        machine: header::Machine::X86_64 as u16,
        version: 1,
        entrypoint: 0,
        program_header_offset: 0,
        section_header_offset: 256,
        flags: 0,
        elf_header_size: 64,
        program_header_size: 0,
        program_header_num: 0,
        section_header_size: 64,
        section_header_num: 7,
        string_table_index: 6,
    }
}

fn ret_o_sections() -> Vec<Section> {
    vec![
        Section {
            name: "".to_string(),
            header: ElfSectionHeader {
                name: 0,
                section_type: 0,
                flags: 0,
                addr: 0,
                offset: 0,
                size: 0,
                link: 0,
                info: 0,
                alignment: 0,
                entry_size: 0,
            },
            data: SectionData::None,
        },
        Section {
            name: ".text".to_string(),
            header: ElfSectionHeader {
                name: 27,
                section_type: section::Type::Progbits as u32,
                flags: (section::Flags::Alloc as u64 | section::Flags::Execinstr as u64),
                addr: 0,
                offset: 0x40,
                size: 0xd,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Raw(vec![72, 199, 199, 42, 0, 0, 0, 49, 192, 176, 60, 15, 5]),
        },
        Section {
            name: ".data".to_string(),
            header: ElfSectionHeader {
                name: 33,
                section_type: section::Type::Progbits as u32,
                flags: (section::Flags::Write as u64 | section::Flags::Alloc as u64),
                addr: 0,
                offset: 0x4d,
                size: 0,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Raw(vec![]),
        },
        Section {
            name: ".bss".to_string(),
            header: ElfSectionHeader {
                name: 39,
                section_type: section::Type::Nobits as u32,
                flags: (section::Flags::Write as u64 | section::Flags::Alloc as u64),
                addr: 0,
                offset: 0x4d,
                size: 0,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Raw(vec![]),
        },
        Section {
            name: ".symtab".to_string(),
            header: ElfSectionHeader {
                name: 1,
                section_type: section::Type::Symtab as u32,
                flags: 0,
                addr: 0,
                offset: 0x50,
                size: 0x78,
                link: 5,
                info: 4,
                alignment: 8,
                entry_size: 0x18,
            },
            data: SectionData::Symbols(vec![
                ElfSymbol {
                    name: 0,
                    info: 0,
                    other: 0,
                    section_index: 0,
                    value: 0,
                    size: 0,
                },
                ElfSymbol {
                    name: 0,
                    info: 3,
                    other: 0,
                    section_index: 1,
                    value: 0,
                    size: 0,
                },
                ElfSymbol {
                    name: 0,
                    info: 3,
                    other: 0,
                    section_index: 2,
                    value: 0,
                    size: 0,
                },
                ElfSymbol {
                    name: 0,
                    info: 3,
                    other: 0,
                    section_index: 3,
                    value: 0,
                    size: 0,
                },
                ElfSymbol {
                    name: 1,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 0,
                    size: 0,
                },
            ]),
        },
        Section {
            name: ".strtab".to_string(),
            header: ElfSectionHeader {
                name: 9,
                section_type: section::Type::Strtab as u32,
                flags: 0,
                addr: 0,
                offset: 0xc8,
                size: 0x8,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Strtab(Strtab::new(b"\x00_start\x00".to_vec())),
        },
        Section {
            name: ".shstrtab".to_string(),
            header: ElfSectionHeader {
                name: 17,
                section_type: section::Type::Strtab as u32,
                flags: 0,
                addr: 0,
                offset: 0xd0,
                size: 0x2c,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Strtab(Strtab::new(
                b"\x00.symtab\x00.strtab\x00.shstrtab\x00.text\x00.data\x00.bss\x00".to_vec(),
            )),
        },
    ]
}

fn ret_header() -> ElfHeader {
    ElfHeader {
        ident: 0x7f454c46020101000000000000000000,
        filetype: header::Type::Exec as u16,
        machine: header::Machine::X86_64 as u16,
        version: 1,
        entrypoint: 0x401000,
        program_header_offset: 64,
        section_header_offset: 4320,
        flags: 0,
        elf_header_size: 64,
        program_header_size: 56,
        program_header_num: 2,
        section_header_size: 64,
        section_header_num: 5,
        string_table_index: 4,
    }
}

fn ret_sections() -> Vec<Section> {
    vec![
        Section {
            name: "".to_string(),
            header: ElfSectionHeader {
                name: 0,
                section_type: 0,
                flags: 0,
                addr: 0,
                offset: 0,
                size: 0,
                link: 0,
                info: 0,
                alignment: 0,
                entry_size: 0,
            },
            data: SectionData::None,
        },
        Section {
            name: ".text".to_string(),
            header: ElfSectionHeader {
                name: 27,
                section_type: section::Type::Progbits as u32,
                flags: (section::Flags::Alloc as u64 | section::Flags::Execinstr as u64),
                addr: 0x401000,
                offset: 0x1000,
                size: 0xd,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Raw(vec![72, 199, 199, 42, 0, 0, 0, 49, 192, 176, 60, 15, 5]),
        },
        Section {
            name: ".symtab".to_string(),
            header: ElfSectionHeader {
                name: 1,
                section_type: section::Type::Symtab as u32,
                flags: 0,
                addr: 0,
                offset: 0x1010,
                size: 0x90,
                link: 3,
                info: 2,
                alignment: 8,
                entry_size: 0x18,
            },
            data: SectionData::Symbols(vec![
                ElfSymbol {
                    name: 0,
                    info: 0,
                    other: 0,
                    section_index: 0,
                    value: 0,
                    size: 0,
                },
                ElfSymbol {
                    name: 0,
                    info: 3,
                    other: 0,
                    section_index: 1,
                    value: 0x401000,
                    size: 0,
                },
                ElfSymbol {
                    name: 6,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 0x401000,
                    size: 0,
                },
                ElfSymbol {
                    name: 1,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 0x402000,
                    size: 0,
                },
                ElfSymbol {
                    name: 13,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 0x402000,
                    size: 0,
                },
                ElfSymbol {
                    name: 20,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 0x402000,
                    size: 0,
                },
            ]),
        },
        Section {
            name: ".strtab".to_string(),
            header: ElfSectionHeader {
                name: 9,
                section_type: section::Type::Strtab as u32,
                flags: 0,
                addr: 0,
                offset: 0x10a0,
                size: 0x19,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Strtab(Strtab::new(
                b"\x00__bss_start\x00_edata\x00_end\x00".to_vec(),
            )),
        },
        Section {
            name: ".shstrtab".to_string(),
            header: ElfSectionHeader {
                name: 17,
                section_type: section::Type::Strtab as u32,
                flags: 0,
                addr: 0,
                offset: 0x10b9,
                size: 0x21,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Strtab(Strtab::new(
                b"\x00.symtab\x00.strtab\x00.shstrtab\x00.text\x00".to_vec(),
            )),
        },
    ]
}

fn ret_segments() -> Vec<ElfProgramHeader> {
    vec![
        ElfProgramHeader {
            typ: segment::Type::Load as u32,
            flags: segment::Flags::R as u32,
            offset: 0,
            virt_addr: 0x400000,
            phys_addr: 0x400000,
            file_size: 0xb0,
            memory_size: 0xb0,
            alignment: 4096,
        },
        ElfProgramHeader {
            typ: segment::Type::Load as u32,
            flags: (segment::Flags::R as u32 | segment::Flags::X as u32),
            offset: 0x1000,
            virt_addr: 0x401000,
            phys_addr: 0x401000,
            file_size: 0xd,
            memory_size: 0xd,
            alignment: 0x1000,
        },
    ]
}

#[test]
fn ret_o_read() {
    let elf = Elf::read_from_file("tests/testcases/ret.o");

    assert_eq!(elf.header, ret_o_header());
    assert_eq!(elf.sections, ret_o_sections());
    assert_eq!(elf.segments, vec![]);
}

#[test]
fn ret_o_write() {
    let elf = Elf {
        header: ret_o_header(),
        sections: ret_o_sections(),
        segments: Vec::new(),
    };

    let expected = fs::read("tests/testcases/ret.o").unwrap();
    let actual = elf.to_bytes();
    assert_eq!(expected, actual);
}

#[test]
fn ret_read() {
    let elf = Elf::read_from_file("tests/testcases/ret");

    assert_eq!(elf.header, ret_header());
    assert_eq!(elf.sections, ret_sections());
    assert_eq!(elf.segments, ret_segments());
}

#[test]
fn ret_write() {
    let elf = Elf {
        header: ret_header(),
        sections: ret_sections(),
        segments: ret_segments(),
    };

    let expected = fs::read("tests/testcases/ret").unwrap();
    let actual = elf.to_bytes();
    assert_eq!(expected, actual);
}
