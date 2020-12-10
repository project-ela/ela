use crate::generator::generate;
use crate::generator::GeneratedData;
use crate::parser::parse;
use crate::tokenizer::tokenize;
use std::fs;

use elfen::elf::Elf;
use elfen::*;

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
        .and_then(parse)
        .and_then(generate)
        .and_then(gen_elf)
}

fn gen_elf(data: GeneratedData) -> Result<Vec<u8>, String> {
    let mut elf = Elf::new();
    elf.header.set_class(header::Class::Class64);
    elf.header.set_data(header::Data::Data2LSB);
    elf.header.set_osabi(header::OSABI::OSABISysV);
    elf.header.set_filetype(header::Type::Rel);
    elf.header.set_machine(header::Machine::X86_64);

    let mut header = section::ElfSectionHeader::default();
    header.set_type(section::Type::Progbits);
    header.set_flags(section::Flags::Alloc);
    header.set_flags(section::Flags::Execinstr);
    header.set_align(1);
    elf.add_section(".text", header, data.program);

    elf.add_symbol("".to_string(), symbol::ElfSymbol::default());
    let mut symbol = symbol::ElfSymbol::default();
    symbol.set_type(symbol::Type::Section);
    symbol.set_index_type(symbol::IndexType::Index(4));
    elf.add_symbol("".to_string(), symbol);

    for sym in data.symbols {
        let mut symbol = symbol::ElfSymbol::default();
        symbol.set_binding(symbol::Binding::Global);
        symbol.set_index_type(symbol::IndexType::Index(4));
        symbol.set_value(sym.addr as u64);
        elf.add_symbol(sym.name, symbol);
    }

    for usym_name in data.unknown_symbols {
        let mut symbol = symbol::ElfSymbol::default();
        symbol.set_binding(symbol::Binding::Global);
        symbol.set_index_type(symbol::IndexType::Index(0));
        elf.add_symbol(usym_name, symbol);
    }

    elf.update_elf_header();
    Ok(elf.to_bytes())
}
