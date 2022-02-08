use std::collections::HashMap;

use crate::{
    backend::gen_code::{SectionName, Symbol, Symbols, Tse},
    frontend::parser::node::{DispNode, InstructionNode, OperandNode, Program, PseudoOp},
};

pub struct SymbolCollector {
    symbols: Symbols,
    tses: Vec<Tse>,
    current_section: SectionName,
    current_symbol: String,

    current_addr: HashMap<SectionName, usize>,
}

impl Symbol {
    fn set_global(&mut self) -> &mut Self {
        self.is_global = true;
        self
    }

    fn set_addr(&mut self, addr: usize) -> &mut Self {
        self.addr = Some(addr);
        self
    }

    fn set_section(&mut self, section: SectionName) -> &mut Self {
        self.section = section;
        self
    }
}

impl SymbolCollector {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            tses: Vec::new(),
            current_section: SectionName::Text,
            current_symbol: String::new(),
            current_addr: HashMap::new(),
        }
    }

    pub fn collect_symbols(mut self, program: &Program) -> (Symbols, Vec<Tse>) {
        for inst in &program.insts {
            self.collect_symbols_in(inst);

            if let InstructionNode::PseudoOp(op, _) = inst {
                match op {
                    PseudoOp::Data => self.current_section = SectionName::Data,
                    PseudoOp::Text => self.current_section = SectionName::Text,
                    _ => {}
                }
            }

            if is_inst(inst) {
                self.inc_addr();
            }
        }

        (self.symbols, self.tses)
    }

    fn collect_symbols_in(&mut self, inst: &InstructionNode) {
        match inst {
            InstructionNode::PseudoOp(PseudoOp::Global, args) => {
                let name = args[0].as_string();
                self.add_symbol(name).set_global();
                self.current_symbol = name.clone();
            }
            InstructionNode::PseudoOp(PseudoOp::Tse, args) => {
                self.tses.push(Tse {
                    symbol_name: self.current_symbol.clone(),
                    offset: *args[0].as_integer() as i64,
                    size: *args[1].as_integer() as u64,
                    align: *args[2].as_integer() as u64,
                });
            }
            InstructionNode::Label(name) => {
                let addr = *self.cur_addr();
                let cur_section = self.current_section.clone();
                self.add_symbol(name)
                    .set_addr(addr)
                    .set_section(cur_section);
            }
            // TODO
            InstructionNode::UnaryOp(_, OperandNode::Label(name)) => {
                self.add_symbol(name);
            }
            InstructionNode::BinaryOp(_, _, OperandNode::Memory(mem)) => {
                if let Some(DispNode::Label(ref name)) = mem.disp {
                    self.add_symbol(name);
                }
            }
            _ => {}
        }
    }

    fn add_symbol(&mut self, name: &str) -> &mut Symbol {
        self.symbols.entry(name.to_string()).or_insert(Symbol {
            name: name.to_string(),
            addr: None,
            is_global: false,
            section: self.current_section.clone(),
        })
    }

    fn inc_addr(&mut self) {
        *self.cur_addr() += 1;
    }

    fn cur_addr(&mut self) -> &mut usize {
        let current_section = self.current_section.clone();
        self.current_addr.entry(current_section).or_insert(0)
    }
}

fn is_inst(inst: &InstructionNode) -> bool {
    matches!(inst,
        InstructionNode::NullaryOp(_)
        | InstructionNode::UnaryOp(_, _)
        | InstructionNode::BinaryOp(_, _, _)
        | InstructionNode::PseudoOp(PseudoOp::Zero, _)
        | InstructionNode::PseudoOp(PseudoOp::Ascii, _)
    )
}
