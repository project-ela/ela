use std::collections::HashMap;

use x86asm::instruction::mnemonic::Mnemonic;
use x86asm::instruction::{operand::memory::Memory, Instruction};
use x86asm::{
    encode,
    instruction::operand::{immediate::Immediate, memory::Displacement, offset::Offset, Operand},
};

use crate::frontend::parser::node::{InstructionNode, OperandNode, PseudoOp};

#[derive(Default)]
struct Generator {
    output: Vec<Instruction>,
    unresolved_jumps: Vec<UnresolvedJump>,
    symbols: HashMap<String, Symbol>,
    relas: Vec<Rela>,
}

type UnresolvedJump = (String, usize);

pub struct Symbol {
    pub name: String,
    pub addr: Option<usize>,
    pub is_global: bool,
}

pub struct Rela {
    pub name: String,
    pub offset: u32,
}

pub struct GeneratedData {
    pub program: Vec<u8>,
    pub symbols: Vec<Symbol>,
    pub relas: Vec<Rela>,
}

pub fn generate(insts: Vec<InstructionNode>) -> Result<GeneratedData, String> {
    let mut generator = Generator::default();

    Ok(GeneratedData {
        program: generator.generate(insts)?,
        symbols: generator.global_symbols(),
        relas: generator.relas,
    })
}

impl Generator {
    fn generate(&mut self, insts: Vec<InstructionNode>) -> Result<Vec<u8>, String> {
        self.process_symbols(&insts);

        for inst in insts {
            self.gen_inst(inst)?;
        }

        self.resolve_jumps();

        Ok(self.output.iter().flat_map(encode::encode).collect())
    }

    fn process_symbols(&mut self, insts: &Vec<InstructionNode>) {
        let mut cur_addr = 0;
        for inst in insts {
            match inst {
                InstructionNode::PseudoOp(PseudoOp::Global, arg) => {
                    self.process_symbol_global(arg);
                }
                InstructionNode::Label { name } => self.process_symbol_label(name, cur_addr),
                InstructionNode::UnaryOp(op, opr1)
                    if matches!(op, Mnemonic::Je | Mnemonic::Jmp | Mnemonic::Call) =>
                {
                    if let OperandNode::Label { name } = opr1 {
                        self.process_symbol_jmp(name);
                    }
                }
                _ => {}
            }

            if matches!(inst, InstructionNode::NullaryOp(_)
                 | InstructionNode::UnaryOp(_, _)
                 | InstructionNode::BinaryOp(_, _, _))
            {
                cur_addr += 1;
            }
        }
    }

    fn process_symbol_global(&mut self, name: &String) {
        match self.symbols.get_mut(name) {
            Some(symbol) => {
                symbol.is_global = true;
            }
            None => {
                self.symbols.insert(
                    name.to_string(),
                    Symbol {
                        name: name.to_string(),
                        addr: None,
                        is_global: true,
                    },
                );
            }
        }
    }

    fn process_symbol_label(&mut self, name: &String, cur_addr: usize) {
        match self.symbols.get_mut(name) {
            Some(symbol) => {
                symbol.addr = Some(cur_addr);
            }
            None => {
                self.symbols.insert(
                    name.to_string(),
                    Symbol {
                        name: name.to_string(),
                        addr: Some(cur_addr),
                        is_global: false,
                    },
                );
            }
        }
    }

    fn process_symbol_jmp(&mut self, name: &String) {
        if !self.symbols.contains_key(name) {
            self.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    addr: None,
                    is_global: false,
                },
            );
        }
    }

    fn gen_inst(&mut self, inst: InstructionNode) -> Result<(), String> {
        match inst {
            InstructionNode::PseudoOp { .. } => {}
            InstructionNode::Label { .. } => {}
            InstructionNode::NullaryOp(op) => self.gen_nullary_inst(op),
            InstructionNode::UnaryOp(op, opr1) => self.gen_unary_inst(op, opr1),
            InstructionNode::BinaryOp(op, opr1, opr2) => self.gen_binary_inst(op, opr1, opr2),
        }
        Ok(())
    }

    fn gen_nullary_inst(&mut self, op: Mnemonic) {
        self.output.push(Instruction::new_nullary(op));
    }

    fn gen_unary_inst(&mut self, op: Mnemonic, opr1: OperandNode) {
        match op {
            Mnemonic::Je | Mnemonic::Jmp | Mnemonic::Call => {
                let name = match opr1 {
                    OperandNode::Label { name } => name,
                    _ => panic!(),
                };

                self.output.push(Instruction::new_unary(
                    op,
                    Operand::Offset(Offset::Off32(0)),
                ));

                self.unresolved_jumps.push((name, self.output.len() - 1));
            }
            _ => {
                let opr1 = self.opr2opr(opr1);
                self.output.push(Instruction::new_unary(op, opr1));
            }
        }
    }

    fn gen_binary_inst(&mut self, op: Mnemonic, opr1: OperandNode, opr2: OperandNode) {
        match op {
            Mnemonic::Mov => self.gen_mov(opr1, opr2),
            _ => {
                let opr1 = self.opr2opr(opr1);
                let opr2 = self.opr2opr(opr2);
                self.output.push(Instruction::new_binary(op, opr1, opr2));
            }
        }
    }

    fn gen_mov(&mut self, opr1: OperandNode, opr2: OperandNode) {
        let opr1 = self.opr2opr(opr1);
        let mut opr2 = self.opr2opr(opr2);
        if let Operand::Immediate(Immediate::Imm8(value)) = opr2 {
            opr2 = Operand::Immediate(Immediate::Imm32(value as i32));
        }
        self.output
            .push(Instruction::new_binary(Mnemonic::Mov, opr1, opr2));
    }

    fn calc_offset(&self, from: usize, to: usize) -> i32 {
        // make from <= to
        let sign = if from < to { 1 } else { -1 };
        let (from, to) = if from < to { (from, to) } else { (to, from) };

        self.output[from..to]
            .iter()
            .map(|inst| encode::encode(inst).len() as i32)
            .sum::<i32>()
            * sign
    }

    fn opr2opr(&mut self, opr: OperandNode) -> Operand {
        match opr {
            OperandNode::Immidiate { value } => {
                if value < 0x80 {
                    Operand::Immediate(Immediate::Imm8(value as i8))
                } else {
                    Operand::Immediate(Immediate::Imm32(value as i32))
                }
            }
            OperandNode::Register { reg } => Operand::Register(reg),
            OperandNode::Memory(mem) => Operand::Memory(Memory::new(
                mem.base,
                mem.disp.map(|disp| {
                    if disp >= -0x80 && disp < 0x80 {
                        Displacement::Disp8(disp as i8)
                    } else {
                        Displacement::Disp32(disp as i32)
                    }
                }),
            )),
            OperandNode::Label { .. } => panic!(),
        }
    }

    fn resolve_jumps(&mut self) {
        for unresolved_jump in &self.unresolved_jumps {
            let symbol = self.symbols.get(&unresolved_jump.0).unwrap();

            let offset = if symbol.is_global || symbol.addr.is_none() {
                0
            } else {
                let inst_index = unresolved_jump.1 + 1;
                let symbol_addr = symbol.addr.unwrap();
                self.calc_offset(inst_index, symbol_addr)
            };

            let inst = self.output.get_mut(unresolved_jump.1).unwrap();
            inst.operand1 = Some(Operand::Offset(Offset::Off32(offset)));

            if symbol.is_global || symbol.addr.is_none() {
                let inst_index = unresolved_jump.1 + 1;
                let rela_offset = self.calc_offset(0, inst_index) as u32 - 4;

                self.relas.push(Rela {
                    name: symbol.name.to_string(),
                    offset: rela_offset,
                });
            }
        }
    }

    fn global_symbols(&mut self) -> Vec<Symbol> {
        let symbols = std::mem::replace(&mut self.symbols, HashMap::new());

        // collect global symbols and convert
        let mut global_symbols: Vec<Symbol> = symbols
            .into_iter()
            .map(|(_, v)| v)
            .filter(|symbol| symbol.is_global || symbol.addr.is_none())
            .map(|mut symbol| {
                symbol.addr = symbol.addr.map(|addr| self.calc_offset(0, addr) as usize);
                symbol
            })
            .collect();

        // sort symbols by offset
        global_symbols.sort_by_key(|symbol| symbol.addr);

        global_symbols
    }
}
