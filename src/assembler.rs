use crate::generator::generate;
use crate::parser::parse;
use crate::tokenizer::tokenize;
use std::fs;

use crate::elf::elf_header;
use crate::elf::section_header;
use crate::elf::symbol;
use crate::elf::*;

pub fn assemble_to_file(input_file: String, output_file: String) -> Result<(), String> {
    match fs::read_to_string(input_file) {
        Ok(source) => {
            let output = assemble(source)?;
            if let Err(err) = fs::write(output_file, output) {
                Err(format!("{}", err))
            } else {
                Ok(())
            }
        }
        Err(err) => Err(format!("{}", err)),
    }
}

pub fn assemble(source: String) -> Result<Vec<u8>, String> {
    tokenize(source)
        .and_then(|tokens| parse(tokens))
        .and_then(|insts| generate(insts))
        .and_then(|program_data| gen_elf(program_data))
}

fn gen_elf(program_data: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut elf = Elf::new();
    elf.elf_header.set_class(elf_header::Class::Class64);
    elf.elf_header.set_data(elf_header::Data::Data2LSB);
    elf.elf_header.set_osabi(elf_header::OSABI::OSABISysV);
    elf.elf_header.set_filetype(elf_header::Type::Rel);
    elf.elf_header.set_machine(elf_header::Machine::X86_64);

    elf.add_section(
        "".to_string(),
        section_header::ElfSectionHeader::new(),
        Vec::new(),
    );

    let mut header = section_header::ElfSectionHeader::new();
    header.set_type(section_header::Type::Progbits);
    header.set_flags(section_header::Flags::Alloc);
    header.set_flags(section_header::Flags::Execinstr);
    header.set_align(1);
    elf.add_section(".text".to_string(), header, program_data);

    elf.add_symbol("".to_string(), symbol::ElfSymbol::new());
    let mut symbol = symbol::ElfSymbol::new();
    symbol.set_type(symbol::Type::Section);
    symbol.set_index_type(symbol::IndexType::Index(1));
    elf.add_symbol("".to_string(), symbol);

    let mut symbol = symbol::ElfSymbol::new();
    symbol.set_binding(symbol::Binding::Global);
    symbol.set_index_type(symbol::IndexType::Index(1));
    elf.add_symbol("main".to_string(), symbol);

    elf.update_elf_header();
    Ok(elf.to_bytes())
}
