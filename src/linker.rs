use std::{
    collections::{HashMap, HashSet},
    fs,
};

use elfen::{
    elf::Elf,
    header,
    rel::Rela,
    section::{self, SectionData},
    strtab::Strtab,
    symbol::{self, Symbol},
};
use section::SectionHeader;

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
    symbol_map: HashMap<SectionPlace, Vec<String>>,
    relas: Vec<RelaSignature>,
    rela_map: HashMap<SectionPlace, Vec<usize>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SectionPlace {
    elf_index: usize,
    section_index: usize,
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
            symbol_map: HashMap::new(),
            relas: Vec::new(),
            rela_map: HashMap::new(),
        }
    }

    fn link(mut self) -> Result<Elf, String> {
        self.init_elf();

        self.load_relas();
        self.load_symbols();

        self.link_sections();

        Ok(self.output_elf)
    }

    fn init_elf(&mut self) {
        let header = &mut self.output_elf.header;
        header.set_class(header::Class::Class64);
        header.set_data(header::Data::Data2LSB);
        header.set_osabi(header::OSABI::OSABISysV);
        header.set_filetype(header::Type::Exec);
        header.set_machine(header::Machine::X86_64);

        self.output_elf
            .add_section("", SectionHeader::default(), SectionData::None);
    }

    fn load_relas(&mut self) {
        for (elf_index, elf) in self.input_elfs.iter_mut().enumerate() {
            let rela_sections: Vec<(usize, Vec<Rela>)> = elf
                .sections
                .iter_mut()
                .filter(|section| section.header.section_type == section::Type::Rela as u32)
                .map(|section| {
                    let section_index = section.header.info as usize;
                    let relas = std::mem::replace(section.data.as_rela_mut().unwrap(), Vec::new());
                    (section_index, relas)
                })
                .collect();

            let symtab_section = elf.get_section(".symtab").unwrap();
            let symbols = symtab_section.data.as_symbols().unwrap();

            let strtab_section = elf.get_section(".strtab").unwrap();
            let strtab = strtab_section.data.as_strtab().unwrap();

            for (section_index, relas) in rela_sections {
                for rela in relas {
                    let symbol = symbols.get(rela.get_symbol() as usize).unwrap();
                    let symbol_name = strtab.get(symbol.name as usize);

                    self.relas.push(RelaSignature { symbol_name, rela });
                    let place = SectionPlace {
                        elf_index,
                        section_index,
                    };
                    self.rela_map
                        .entry(place)
                        .or_insert(Vec::new())
                        .push(self.relas.len() - 1);
                }
            }
        }
    }

    fn load_symbols(&mut self) {
        for (elf_index, elf) in self.input_elfs.iter_mut().enumerate() {
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
                                name: symbol_name.clone(),
                                symbol,
                            },
                        );

                        let place = SectionPlace {
                            elf_index,
                            section_index: symbol.section_index as usize,
                        };
                        self.symbol_map
                            .entry(place)
                            .or_insert(Vec::new())
                            .push(symbol_name);
                    }
                }
            }
        }
    }

    fn link_sections(&mut self) {
        for section_name in self.list_sections_to_alloc() {
            let new_section_index = self.output_elf.sections.len();
            let mut section_header = None;
            let mut linked_data: Vec<u8> = Vec::new();
            for (elf_index, elf) in self.input_elfs.iter_mut().enumerate() {
                let section_index = if let Some(index) = elf.find_section(&section_name) {
                    index
                } else {
                    continue;
                };

                let section = elf.sections.get(section_index).unwrap();
                section_header = Some(section.header);
                let offset = linked_data.len() as u64;
                let section_data = section.data.as_raw().unwrap();
                linked_data.extend(section_data);

                let place = SectionPlace {
                    elf_index,
                    section_index,
                };
                let new_place = SectionPlace {
                    elf_index: 0,
                    section_index: new_section_index,
                };

                // offset symbols
                if let Some(symbol_names) = self.symbol_map.remove(&place) {
                    for symbol_name in &symbol_names {
                        let symbol_sig = self.global_symbols.get_mut(symbol_name).unwrap();
                        symbol_sig.symbol.value += offset;
                    }
                    self.symbol_map
                        .entry(new_place.clone())
                        .or_insert(Vec::new())
                        .extend(symbol_names);
                }

                // offset relas
                if let Some(rela_indices) = self.rela_map.remove(&place) {
                    for rela_index in &rela_indices {
                        let rela_sig = self.relas.get_mut(*rela_index).unwrap();
                        rela_sig.rela.offset += offset;
                    }

                    self.rela_map
                        .entry(new_place)
                        .or_insert(Vec::new())
                        .extend(rela_indices);
                }
            }

            if linked_data.len() == 0 {
                continue;
            }

            self.output_elf.add_section(
                &section_name,
                section_header.unwrap(),
                SectionData::Raw(linked_data),
            );
        }
    }

    fn list_sections_to_alloc(&self) -> HashSet<String> {
        let mut section_names = HashSet::new();
        for elf in &self.input_elfs {
            for section in &elf.sections {
                if section.header.flags & section::Flags::Alloc as u64 != 0 {
                    section_names.insert(section.name.clone());
                }
            }
        }
        section_names
    }
}
