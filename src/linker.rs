use std::{collections::HashMap, fs};

use elfen::{
    elf::Elf,
    rel::Rela,
    section,
    strtab::Strtab,
    symbol::{self, Symbol},
};

pub fn link_to_files(input_files: Vec<String>, output_file: String) -> Result<(), String> {
    let input_elfs = input_files
        .into_iter()
        .map(|path| Elf::read_from_file(&path))
        .collect();

    let output_elf = link(input_elfs)?;

    let elf_bytes = output_elf.to_bytes();
    fs::write(output_file, elf_bytes).unwrap();

    Ok(())
}

pub fn link(input_elfs: Vec<Elf>) -> Result<Elf, String> {
    let linker = Linker::new(input_elfs);
    let output_elf = linker.link()?;
    Ok(output_elf)
}

struct Linker {
    input_elfs: Vec<Elf>,
    output_elf: Elf,

    global_symbols: HashMap<String, SymbolSignature>,
    relas: Vec<RelaSignature>,
}

#[derive(Debug)]
struct SymbolSignature {
    name: String,
    symbol: Symbol,
}

#[derive(Debug)]
struct RelaSignature {
    symbol_name: String,
    rela: Rela,
}

impl Linker {
    fn new(input_elfs: Vec<Elf>) -> Self {
        Self {
            input_elfs,
            output_elf: Elf::default(),
            global_symbols: HashMap::new(),
            relas: Vec::new(),
        }
    }

    fn link(mut self) -> Result<Elf, String> {
        self.load_relas();
        self.load_symbols();

        Ok(self.output_elf)
    }

    fn load_relas(&mut self) {
        for elf in self.input_elfs.iter_mut() {
            let relas: Vec<Rela> = elf
                .sections
                .iter_mut()
                .filter(|section| section.header.section_type == section::Type::Rela as u32)
                .flat_map(|section| {
                    std::mem::replace(section.data.as_rela_mut().unwrap(), Vec::new())
                })
                .collect();

            let symtab_section = elf.get_section(".symtab").unwrap();
            let symbols = symtab_section.data.as_symbols().unwrap();

            let strtab_section = elf.get_section(".strtab").unwrap();
            let strtab = strtab_section.data.as_strtab().unwrap();

            for rela in relas {
                let symbol = symbols.get(rela.get_symbol() as usize).unwrap();
                let symbol_name = strtab.get(symbol.name as usize);

                self.relas.push(RelaSignature { symbol_name, rela });
            }
        }
    }

    fn load_symbols(&mut self) {
        for elf in self.input_elfs.iter_mut() {
            let symtab_section = elf.get_section_mut(".symtab").unwrap();
            let symtab_data = symtab_section.data.as_symbols_mut().unwrap();
            let symbols = std::mem::replace(symtab_data, Vec::new());

            let strtab_section = elf.get_section_mut(".strtab").unwrap();
            let strtab_data = strtab_section.data.as_strtab_mut().unwrap();
            let strtab = std::mem::replace(strtab_data, Strtab::default());
            for symbol in symbols {
                if symbol.get_binding() != Some(symbol::Binding::Global) {
                    continue;
                }
                let symbol_name = strtab.get(symbol.name as usize);
                match self.global_symbols.get(&symbol_name) {
                    Some(symbol_sig) => {
                        if symbol_sig.symbol.section_index == 0 {
                            panic!("duplicate symbol: {}", symbol_name);
                        }
                    }
                    None => {
                        self.global_symbols.insert(
                            symbol_name.clone(),
                            SymbolSignature {
                                name: symbol_name,
                                symbol,
                            },
                        );
                    }
                }
            }
        }
    }
}
