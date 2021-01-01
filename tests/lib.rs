use elfen::{
    elf::Elf,
    header::{self, Header},
    rel::Rela,
    section::{self, Section, SectionData, SectionHeader},
    segment::{self, ProgramHeader},
    strtab::Strtab,
    symbol::Symbol,
};
use pretty_assertions::assert_eq;
use std::fs;

fn ret_o_header() -> Header {
    Header {
        ident: 0x7f454c46020101000000000000000000,
        filetype: header::Type::Rel.into(),
        machine: header::Machine::X86_64.into(),
        version: 1,
        entrypoint: 0,
        program_header_offset: 0,
        section_header_offset: 328,
        flags: 0,
        elf_header_size: 64,
        program_header_size: 0,
        program_header_num: 0,
        section_header_size: 64,
        section_header_num: 8,
        string_table_index: 7,
    }
}

fn ret_o_sections() -> Vec<Section> {
    vec![
        Section {
            name: "".to_string(),
            header: SectionHeader {
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
            header: SectionHeader {
                name: 32,
                section_type: section::Type::Progbits.into(),
                flags: (Into::<u64>::into(section::Flags::Alloc)
                    | Into::<u64>::into(section::Flags::Execinstr)),
                addr: 0,
                offset: 0x40,
                size: 0x13,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Raw(vec![
                232, 0, 0, 0, 0, 72, 199, 199, 42, 0, 0, 0, 49, 192, 176, 60, 15, 5, 195,
            ]),
        },
        Section {
            name: ".rela.text".to_string(),
            header: SectionHeader {
                name: 27,
                section_type: section::Type::Rela.into(),
                flags: section::Flags::InfoLink.into(),
                addr: 0,
                offset: 0xf8,
                size: 0x18,
                link: 5,
                info: 1,
                alignment: 8,
                entry_size: 24,
            },
            data: SectionData::Rela(vec![Rela {
                offset: 1,
                info: 0x500000004,
                addend: -4,
            }]),
        },
        Section {
            name: ".data".to_string(),
            header: SectionHeader {
                name: 38,
                section_type: section::Type::Progbits.into(),
                flags: (Into::<u64>::into(section::Flags::Write)
                    | Into::<u64>::into(section::Flags::Alloc)),
                addr: 0,
                offset: 0x53,
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
            header: SectionHeader {
                name: 44,
                section_type: section::Type::Nobits.into(),
                flags: (Into::<u64>::into(section::Flags::Write)
                    | Into::<u64>::into(section::Flags::Alloc)),
                addr: 0,
                offset: 0x53,
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
            header: SectionHeader {
                name: 1,
                section_type: section::Type::Symtab.into(),
                flags: 0,
                addr: 0,
                offset: 0x58,
                size: 0x90,
                link: 6,
                info: 4,
                alignment: 8,
                entry_size: 0x18,
            },
            data: SectionData::Symbols(vec![
                Symbol {
                    name: 0,
                    info: 0,
                    other: 0,
                    section_index: 0,
                    value: 0,
                    size: 0,
                },
                Symbol {
                    name: 0,
                    info: 3,
                    other: 0,
                    section_index: 1,
                    value: 0,
                    size: 0,
                },
                Symbol {
                    name: 0,
                    info: 3,
                    other: 0,
                    section_index: 3,
                    value: 0,
                    size: 0,
                },
                Symbol {
                    name: 0,
                    info: 3,
                    other: 0,
                    section_index: 4,
                    value: 0,
                    size: 0,
                },
                Symbol {
                    name: 1,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 0,
                    size: 0,
                },
                Symbol {
                    name: 8,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 18,
                    size: 0,
                },
            ]),
        },
        Section {
            name: ".strtab".to_string(),
            header: SectionHeader {
                name: 9,
                section_type: section::Type::Strtab.into(),
                flags: 0,
                addr: 0,
                offset: 0xe8,
                size: 0xd,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Strtab(Strtab::new(b"\x00_start\x00main\x00".to_vec())),
        },
        Section {
            name: ".shstrtab".to_string(),
            header: SectionHeader {
                name: 17,
                section_type: section::Type::Strtab.into(),
                flags: 0,
                addr: 0,
                offset: 0x110,
                size: 0x31,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Strtab(Strtab::new(
                b"\x00.symtab\x00.strtab\x00.shstrtab\x00.rela.text\x00.data\x00.bss\x00".to_vec(),
            )),
        },
    ]
}

fn ret_header() -> Header {
    Header {
        ident: 0x7f454c46020101000000000000000000,
        filetype: header::Type::Exec.into(),
        machine: header::Machine::X86_64.into(),
        version: 1,
        entrypoint: 0x401000,
        program_header_offset: 64,
        section_header_offset: 4352,
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
            header: SectionHeader {
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
            header: SectionHeader {
                name: 27,
                section_type: section::Type::Progbits.into(),
                flags: (Into::<u64>::into(section::Flags::Alloc)
                    | Into::<u64>::into(section::Flags::Execinstr)),
                addr: 0x401000,
                offset: 0x1000,
                size: 0x13,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Raw(vec![
                232, 13, 0, 0, 0, 72, 199, 199, 42, 0, 0, 0, 49, 192, 176, 60, 15, 5, 195,
            ]),
        },
        Section {
            name: ".symtab".to_string(),
            header: SectionHeader {
                name: 1,
                section_type: section::Type::Symtab.into(),
                flags: 0,
                addr: 0,
                offset: 0x1018,
                size: 0xa8,
                link: 3,
                info: 2,
                alignment: 8,
                entry_size: 0x18,
            },
            data: SectionData::Symbols(vec![
                Symbol {
                    name: 0,
                    info: 0,
                    other: 0,
                    section_index: 0,
                    value: 0,
                    size: 0,
                },
                Symbol {
                    name: 0,
                    info: 3,
                    other: 0,
                    section_index: 1,
                    value: 0x401000,
                    size: 0,
                },
                Symbol {
                    name: 6,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 0x401000,
                    size: 0,
                },
                Symbol {
                    name: 1,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 0x402000,
                    size: 0,
                },
                Symbol {
                    name: 13,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 0x401012,
                    size: 0,
                },
                Symbol {
                    name: 18,
                    info: 16,
                    other: 0,
                    section_index: 1,
                    value: 0x402000,
                    size: 0,
                },
                Symbol {
                    name: 25,
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
            header: SectionHeader {
                name: 9,
                section_type: section::Type::Strtab.into(),
                flags: 0,
                addr: 0,
                offset: 0x10c0,
                size: 0x1e,
                link: 0,
                info: 0,
                alignment: 1,
                entry_size: 0,
            },
            data: SectionData::Strtab(Strtab::new(
                b"\x00__bss_start\x00main\x00_edata\x00_end\x00".to_vec(),
            )),
        },
        Section {
            name: ".shstrtab".to_string(),
            header: SectionHeader {
                name: 17,
                section_type: section::Type::Strtab.into(),
                flags: 0,
                addr: 0,
                offset: 0x10de,
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

fn ret_segments() -> Vec<ProgramHeader> {
    vec![
        ProgramHeader {
            typ: segment::Type::Load.into(),
            flags: segment::Flags::R.into(),
            offset: 0,
            virt_addr: 0x400000,
            phys_addr: 0x400000,
            file_size: 0xb0,
            memory_size: 0xb0,
            alignment: 4096,
        },
        ProgramHeader {
            typ: segment::Type::Load.into(),
            flags: (Into::<u32>::into(segment::Flags::R) | Into::<u32>::into(segment::Flags::X)),
            offset: 0x1000,
            virt_addr: 0x401000,
            phys_addr: 0x401000,
            file_size: 0x13,
            memory_size: 0x13,
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
