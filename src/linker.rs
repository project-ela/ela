use std::{
    collections::{HashMap, HashSet},
    fs,
    mem::size_of,
};

use elfen::{
    elf::Elf,
    header::{self, Header},
    rel::{self, Rela},
    section::{self, SectionData, SectionHeader},
    segment::{self, ProgramHeader},
    strtab::Strtab,
    symbol::{self, Symbol},
    tse::Tse,
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
    symbol_map: HashMap<SectionPlace, Vec<String>>,

    relas: Vec<RelaSignature>,
    rela_map: HashMap<SectionPlace, Vec<usize>>,

    tses: Vec<TseSignature>,
    symbol_indices: HashMap<String, usize>,

    section_offsets: HashMap<usize, u64>,
}

const BASE_ADDRESS: u64 = 0x400000;
const PAGE_SIZE: u64 = 0x1000;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct SectionPlace {
    // elf_index = 0 is reserved for output_elf
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

#[derive(Debug)]
struct TseSignature {
    symbol_name: String,
    tse: Tse,
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
            tses: Vec::new(),
            symbol_indices: HashMap::new(),
            section_offsets: HashMap::new(),
        }
    }

    fn link(mut self) -> Result<Elf, String> {
        self.init_elf();

        self.load_tses();
        self.load_relas();
        self.load_symbols();

        self.link_sections();
        self.layout();
        self.resolve_relas();

        self.gen_symtab_strtab();
        self.gen_tse_info();
        self.gen_shstrtab();

        self.layout();
        self.finalize_elf();

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

    fn load_tses(&mut self) {
        for elf in self.input_elfs.iter_mut() {
            let tses = if let Some(section) = elf.get_section_mut(".tse_info") {
                std::mem::take(section.data.as_tse_mut().unwrap())
            } else {
                continue;
            };

            let symtab_section = elf.get_section(".symtab").unwrap();
            let symbols = symtab_section.data.as_symbols().unwrap();

            let strtab_section = elf.get_section(".strtab").unwrap();
            let strtab = strtab_section.data.as_strtab().unwrap();

            for tse in tses {
                let symbol = symbols.get(tse.symbol_index as usize).unwrap();
                let symbol_name = strtab.get(symbol.name as usize);
                self.tses.push(TseSignature { symbol_name, tse });
            }
        }
    }

    fn load_relas(&mut self) {
        for (elf_index, elf) in self.input_elfs.iter_mut().enumerate() {
            let rela_sections: Vec<(usize, Vec<Rela>)> = elf
                .sections
                .iter_mut()
                .filter(|section| section.header.get_type() == section::Type::Rela)
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
                        elf_index: elf_index + 1,
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
                if symbol.get_binding() != symbol::Binding::Global {
                    continue;
                }
                let symbol_name = strtab.get(symbol.name as usize);

                if let Some(symbol_sig) = self.global_symbols.get(&symbol_name) {
                    if symbol.get_index_type() == symbol::IndexType::Undef {
                        continue;
                    }
                    if symbol_sig.symbol.get_index_type() != symbol::IndexType::Undef {
                        panic!("duplicate symbol: {}", symbol_name);
                    }
                }

                self.global_symbols.insert(
                    symbol_name.clone(),
                    SymbolSignature {
                        name: symbol_name.clone(),
                        symbol,
                    },
                );

                let place = SectionPlace {
                    elf_index: elf_index + 1,
                    section_index: symbol.section_index as usize,
                };
                self.symbol_map
                    .entry(place)
                    .or_insert(Vec::new())
                    .push(symbol_name);
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
                    elf_index: elf_index + 1,
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
                        symbol_sig.symbol.section_index = new_section_index as u16;
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

    fn list_sections_to_alloc(&self) -> Vec<String> {
        let mut section_names = HashSet::new();
        for elf in &self.input_elfs {
            for section in &elf.sections {
                if section::Flags::Alloc.contained_in(section.header.flags) {
                    section_names.insert(section.name.clone());
                }
            }
        }
        let mut symbol_names: Vec<String> = section_names.into_iter().collect();
        symbol_names.sort();
        symbol_names
    }

    fn layout(&mut self) {
        self.output_elf.segments.clear();

        let mut cur_offset = size_of::<Header>() as u64;
        for (section_index, section) in self.output_elf.sections.iter_mut().enumerate() {
            // skip null section
            if section_index == 0 {
                continue;
            }
            let shdr = &mut section.header;

            shdr.size = section.data.len() as u64;

            if section::Flags::Alloc.contained_in(shdr.flags) {
                let mut phdr = Self::gen_segment(&shdr);
                shdr.offset = Self::align(cur_offset, phdr.alignment);
                phdr.offset = shdr.offset;

                shdr.addr = BASE_ADDRESS + shdr.offset;
                phdr.virt_addr = shdr.addr;
                phdr.phys_addr = shdr.addr;

                self.output_elf.segments.push(phdr);
            } else {
                shdr.offset = Self::align(cur_offset, shdr.alignment);
            }
            cur_offset = shdr.offset + shdr.size;

            let offset = if shdr.addr != 0 {
                shdr.addr
            } else {
                shdr.offset
            };
            self.section_offsets.insert(section_index, offset);
        }
    }

    fn resolve_relas(&mut self) {
        for (section_index, section) in self.output_elf.sections.iter_mut().enumerate() {
            let place = SectionPlace {
                elf_index: 0,
                section_index,
            };
            let rela_indices = if let Some(indices) = self.rela_map.get(&place) {
                indices
            } else {
                continue;
            };

            for rela_index in rela_indices {
                let rela_sig = self.relas.get_mut(*rela_index).unwrap();
                let target_symbol = self
                    .global_symbols
                    .get(&rela_sig.symbol_name)
                    .unwrap()
                    .symbol;

                let addr_from = rela_sig.rela.offset as i32;
                let addr_to = target_symbol.value as i32;

                let mut diff = match rela_sig.rela.get_type() {
                    rel::Type::Pc32 => {
                        let offset_from = *self.section_offsets.get(&section_index).unwrap() as i32;
                        let sym_idx: u16 = target_symbol.get_index_type().into();
                        let offset_to = *self.section_offsets.get(&sym_idx.into()).unwrap() as i32;

                        (addr_to + offset_to) - (addr_from + offset_from)
                    }
                    rel::Type::Plt32 => addr_to - addr_from,
                    _ => panic!(),
                };
                diff += rela_sig.rela.addend as i32;

                let code_index = addr_from as usize;
                let section_data = section.data.as_raw_mut().unwrap();
                for (i, value) in diff.to_le_bytes().iter().enumerate() {
                    section_data[(code_index + i)] = *value;
                }
            }
        }
    }

    fn gen_segment(shdr: &SectionHeader) -> ProgramHeader {
        let mut phdr = ProgramHeader::default();
        phdr.set_type(segment::Type::Load);
        phdr.set_flags(segment::Flags::R);
        phdr.alignment = PAGE_SIZE;

        phdr.file_size = shdr.size;
        phdr.memory_size = shdr.size;

        if section::Flags::Execinstr.contained_in(shdr.flags) {
            phdr.set_flags(segment::Flags::X);
        }
        if section::Flags::Write.contained_in(shdr.flags) {
            phdr.set_flags(segment::Flags::W);
        }

        phdr
    }

    fn align(x: u64, align: u64) -> u64 {
        (x + align - 1) & !(align - 1)
    }

    fn gen_symtab_strtab(&mut self) {
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut strtab = Strtab::default();

        symbols.push(Symbol::default());
        strtab.insert("".into());

        let mut symbol_sigs: Vec<&SymbolSignature> = self.global_symbols.values().collect();
        symbol_sigs.sort_by_key(|sig| sig.symbol.value);

        for symbol_sig in symbol_sigs {
            let mut symbol = symbol_sig.symbol.clone();
            let symbol_name = symbol_sig.name.clone();

            let symbol_section_index = symbol.section_index as usize;
            symbol.value += self.section_offsets.get(&symbol_section_index).unwrap();
            symbol.name = strtab.insert(symbol_name.clone()) as u32;

            symbols.push(symbol);
            self.symbol_indices.insert(symbol_name, symbols.len() - 1);
        }

        // generate symtab
        {
            let mut header = SectionHeader::default();
            header.set_type(section::Type::Symtab);
            header.entry_size = size_of::<Symbol>() as u64;
            header.link = self.output_elf.sections.len() as u32 + 1;
            header.alignment = 8;
            let num_local_symbols = symbols
                .iter()
                .filter(|symbol| symbol.get_binding() == symbol::Binding::Local)
                .count();
            header.info = num_local_symbols as u32;

            let data = SectionData::Symbols(symbols);

            self.output_elf.add_section(".symtab", header, data);
        }

        // generate strtab
        {
            let mut header = SectionHeader::default();
            header.set_type(section::Type::Strtab);
            header.alignment = 1;

            let data = SectionData::Strtab(strtab);

            self.output_elf.add_section(".strtab", header, data);
        }
    }

    fn gen_tse_info(&mut self) {
        let mut header = SectionHeader::default();
        header.set_type(section::Type::Progbits);
        header.entry_size = size_of::<Tse>() as u64;
        header.alignment = 8;

        let mut tses = Vec::new();
        for tse_sig in std::mem::take(&mut self.tses) {
            let mut tse = tse_sig.tse;
            tse.symbol_index = *self.symbol_indices.get(&tse_sig.symbol_name).unwrap() as u64;
            tses.push(tse);
        }

        let data = SectionData::Tse(tses);

        self.output_elf.add_section(".tse_info", header, data);
    }

    fn gen_shstrtab(&mut self) {
        let mut header = SectionHeader::default();
        header.set_type(section::Type::Strtab);
        header.alignment = 1;

        let mut strtab = Strtab::default();
        strtab.insert("".into());
        for section in self.output_elf.sections.as_mut_slice() {
            section.header.name = strtab.insert(section.name.clone()) as u32;
        }
        header.name = strtab.insert(".shstrtab".into()) as u32;

        let data = SectionData::Strtab(strtab);

        self.output_elf.add_section(".shstrtab", header, data);
    }

    fn finalize_elf(&mut self) {
        self.output_elf.update_header();

        let addr_of_text = self.output_elf.get_section(".text").unwrap().header.addr;
        let entrypoint = self.find_symbol("_start").unwrap_or(addr_of_text);
        self.output_elf.header.entrypoint = entrypoint;
    }

    fn find_symbol(&self, name: &str) -> Option<u64> {
        let symbol_sig = self.global_symbols.get(name)?;
        let symbol = symbol_sig.symbol;

        let symbol_section_index = symbol.section_index as usize;
        let symbol_offset = self.section_offsets.get(&symbol_section_index).unwrap();

        Some(symbol.value + symbol_offset)
    }
}
