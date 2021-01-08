use std::{collections::HashMap, mem::size_of};

use elfen::{
    elf::Elf,
    header,
    rel::{self, Rela},
    section::{self, SectionData, SectionHeader},
    strtab::Strtab,
    symbol::{self, Symbol},
};

use crate::{backend::gen_code::GeneratedData, common::error::Error};

struct ElfGen {
    elf: Elf,
    data: GeneratedData,
    // holds symbol name and symbol index
    symbols: HashMap<String, usize>,
}

pub fn generate(data: GeneratedData) -> Result<Elf, Error> {
    let elfgen = ElfGen::new(data);
    elfgen.generate()
}

impl ElfGen {
    fn new(data: GeneratedData) -> Self {
        Self {
            elf: Elf::default(),
            data,
            symbols: HashMap::new(),
        }
    }

    fn generate(mut self) -> Result<Elf, Error> {
        self.gen_header();
        self.gen_sections();
        self.elf.update_section_headers();
        self.elf.update_header();
        Ok(self.elf)
    }

    fn gen_header(&mut self) {
        let header = &mut self.elf.header;
        header.set_class(header::Class::Class64);
        header.set_data(header::Data::Data2LSB);
        header.set_osabi(header::OSABI::OSABISysV);
        header.set_filetype(header::Type::Rel);
        header.set_machine(header::Machine::X86_64);
    }

    fn gen_sections(&mut self) {
        // add null section
        self.elf
            .add_section("", SectionHeader::default(), SectionData::None);

        self.gen_text();
        self.gen_symtab_strtab();
        self.gen_rela();
        self.gen_shstrtab();
    }

    fn gen_text(&mut self) {
        let mut header = SectionHeader::default();
        header.set_type(section::Type::Progbits);
        header.set_flags(section::Flags::Alloc);
        header.set_flags(section::Flags::Execinstr);
        header.alignment = 1;

        let program = std::mem::replace(&mut self.data.program, Vec::new());
        let data = SectionData::Raw(program);

        self.elf.add_section(".text", header, data);
    }

    fn gen_symtab_strtab(&mut self) {
        let mut symtab_header = SectionHeader::default();
        symtab_header.set_type(section::Type::Symtab);
        symtab_header.entry_size = size_of::<Symbol>() as u64;
        symtab_header.alignment = 8;

        let mut strtab_header = SectionHeader::default();
        strtab_header.set_type(section::Type::Strtab);
        strtab_header.alignment = 1;

        let mut symbols = Vec::new();
        let mut strtab = Strtab::default();

        // add null symbol
        symbols.push(Symbol::default());
        strtab.insert("".into());

        // add section symbol
        let text_section_index = self.elf.find_section(".text").unwrap() as u16;
        let mut symbol_text_section = Symbol::default();
        symbol_text_section.set_type(symbol::Type::Section);
        symbol_text_section.set_binding(symbol::Binding::Local);
        symbol_text_section.set_index_type(symbol::IndexType::Index(text_section_index));
        symbols.push(symbol_text_section);

        // add symbols
        for symbol_data in &self.data.symbols {
            let mut symbol = Symbol::default();
            symbol.name = strtab.insert(symbol_data.name.clone()) as u32;
            symbol.set_binding(symbol::Binding::Global);
            match symbol_data.addr {
                Some(addr) => {
                    symbol.set_index_type(symbol::IndexType::Index(text_section_index));
                    symbol.value = addr as u64;
                }
                None => symbol.set_index_type(symbol::IndexType::Undef),
            }
            symbols.push(symbol);

            self.symbols
                .insert(symbol_data.name.clone(), symbols.len() - 1);
        }

        // set symtab's info/link value
        let num_local_symbols = symbols
            .iter()
            .filter(|symbol| symbol.get_binding() == symbol::Binding::Local)
            .count();
        symtab_header.info = num_local_symbols as u32;
        symtab_header.link = self.elf.sections.len() as u32 + 1;

        let symtab_data = SectionData::Symbols(symbols);
        let strtab_data = SectionData::Strtab(strtab);

        self.elf.add_section(".symtab", symtab_header, symtab_data);
        self.elf.add_section(".strtab", strtab_header, strtab_data);
    }

    fn gen_rela(&mut self) {
        let mut header = SectionHeader::default();
        header.set_type(section::Type::Rela);
        header.set_flags(section::Flags::InfoLink);
        header.entry_size = size_of::<Rela>() as u64;
        header.alignment = 8;

        let symtab_section_index = self.elf.find_section(".symtab").unwrap();
        header.link = symtab_section_index as u32;
        let text_section_index = self.elf.find_section(".text").unwrap();
        header.info = text_section_index as u32;

        let mut relas = Vec::new();
        for rela_data in &self.data.relas {
            let mut rela = Rela::default();
            rela.offset = rela_data.offset as u64;
            let symbol_index = self
                .symbols
                .get(&rela_data.name)
                .expect(&format!("cannot find symbol '{}'", rela_data.name));
            rela.set_info(*symbol_index as u64, rel::Type::Plt32);
            rela.addend = -4;
            relas.push(rela);
        }

        let data = SectionData::Rela(relas);

        self.elf.add_section(".rela.text", header, data);
    }

    fn gen_shstrtab(&mut self) {
        let mut header = SectionHeader::default();
        header.set_type(section::Type::Strtab);
        header.alignment = 1;

        let mut strtab = Strtab::default();
        strtab.insert("".into());
        for section in self.elf.sections.as_mut_slice() {
            section.header.name = strtab.insert(section.name.clone()) as u32;
        }
        header.name = strtab.insert(".shstrtab".into()) as u32;

        let data = SectionData::Strtab(strtab);

        self.elf.add_section(".shstrtab", header, data);
    }
}
