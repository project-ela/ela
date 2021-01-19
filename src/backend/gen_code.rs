pub mod collect_symbol;
pub mod generate_code;
pub mod resolve_symbol;

use std::collections::HashMap;

use x86asm::{encode, instruction::Instruction};

use crate::{
    backend::gen_code::{
        collect_symbol::SymbolCollector,
        generate_code::CodeGen,
        resolve_symbol::{list_global_symbols, resolve_symbol},
    },
    common::error::Error,
    frontend::parser::node::Program,
};

pub fn generate(program: Program) -> Result<Object, Error> {
    let collector = SymbolCollector::new();
    let symbols = collector.collect_symbols(&program);

    let generator = CodeGen::new();
    let mut codes = generator.gen_program(program);

    Ok(Object {
        sections: gen_sections(&symbols, &mut codes),
        global_symbols: list_global_symbols(symbols, &codes),
    })
}

fn gen_sections(symbols: &Symbols, codes: &mut Codes) -> Vec<Section> {
    let mut sections = Vec::new();
    for (section_name, code) in codes.iter_mut() {
        let section_relas = resolve_symbol(&symbols, code);

        let section_data: Vec<u8> = code
            .items
            .iter()
            .flat_map(|item| encode_item(&item))
            .collect();

        sections.push(Section {
            name: section_name.clone(),
            data: section_data,
            relas: section_relas,
        })
    }
    sections
}

fn encode_item(item: &CodeItem) -> Vec<u8> {
    match item {
        CodeItem::Raw(data) => data.clone(),
        CodeItem::Inst(inst) => encode::encode(inst),
    }
}

type Symbols = HashMap<String, Symbol>;
type Codes = HashMap<SectionName, Code>;

#[derive(Debug)]
pub struct Object {
    pub sections: Vec<Section>,
    pub global_symbols: Vec<Symbol>,
}

#[derive(Debug)]
pub struct Section {
    pub name: SectionName,
    pub data: Vec<u8>,
    pub relas: Vec<Rela>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub addr: Option<usize>,
    pub is_global: bool,
    pub section: SectionName,
}

#[derive(Debug)]
pub struct Rela {
    pub name: String,
    pub offset: u32,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum SectionName {
    Data,
    Text,
}

impl SectionName {
    pub fn as_str(&self) -> &str {
        match self {
            SectionName::Data => ".data",
            SectionName::Text => ".text",
        }
    }
}

#[derive(Default, Debug)]
pub struct Code {
    items: Vec<CodeItem>,
    unresolved_jumps: Vec<UnresolvedJump>,
}

#[derive(Debug)]
enum CodeItem {
    Raw(Vec<u8>),
    Inst(Instruction),
}

#[derive(Debug)]
struct UnresolvedJump {
    label_name: String,
    item_index: usize,
}
