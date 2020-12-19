use std::collections::{HashMap, HashSet};

use x86asm::instruction::mnemonic::Mnemonic;
use x86asm::instruction::{operand::memory::Memory, Instruction};
use x86asm::{
    encode,
    instruction::operand::{immediate::Immediate, memory::Displacement, offset::Offset, Operand},
};

use crate::frontend::parser::node::{InstructionNode, OperandNode};

#[derive(Default)]
struct Generator {
    output: Vec<Instruction>,
    labels: HashMap<String, usize>,
    global_symbols: HashSet<String>,
    unknown_symbols: HashSet<String>,
    unresolved_jumps: Vec<UnresolvedJump>,
}

type UnresolvedJump = (String, usize);

pub struct GlobalSymbol {
    pub name: String,
    pub addr: u32,
}

pub struct GeneratedData {
    pub program: Vec<u8>,
    pub symbols: Vec<GlobalSymbol>,
    pub unknown_symbols: HashSet<String>,
}

pub fn generate(insts: Vec<InstructionNode>) -> Result<GeneratedData, String> {
    let mut generator = Generator::default();

    Ok(GeneratedData {
        program: generator.generate(insts)?,
        symbols: generator.global_symbols(),
        unknown_symbols: generator.unknown_symbols,
    })
}

impl Generator {
    fn generate(&mut self, insts: Vec<InstructionNode>) -> Result<Vec<u8>, String> {
        for inst in insts {
            self.gen_inst(inst)?;
        }
        self.resolve_jump()?;

        Ok(self.output.iter().flat_map(encode::encode).collect())
    }

    fn global_symbols(&self) -> Vec<GlobalSymbol> {
        let mut syms = Vec::new();

        for sym_name in &self.global_symbols {
            if let Some(idx) = self.labels.get(sym_name) {
                let addr = self.calc_offset(0, *idx);
                syms.push(GlobalSymbol {
                    name: sym_name.clone(),
                    addr: addr as u32,
                });
            }
        }

        syms
    }

    fn gen_inst(&mut self, inst: InstructionNode) -> Result<(), String> {
        match inst {
            InstructionNode::PseudoOp { name, arg } => match name.as_str() {
                ".global" => {
                    self.global_symbols.insert(arg);
                }
                _ => {}
            },
            InstructionNode::Label { name } => {
                let idx = self.output.len();
                self.labels.insert(name, idx);
            }
            InstructionNode::NullaryOp(op) => self.output.push(Instruction::new_nullary(op)),
            InstructionNode::UnaryOp(op, opr1) => match op {
                Mnemonic::Je | Mnemonic::Jmp | Mnemonic::Call => self.gen_jmp(op, opr1),
                _ => {
                    let opr1 = self.opr2opr(opr1);
                    self.output.push(Instruction::new_unary(op, opr1));
                }
            },
            InstructionNode::BinaryOp(op, opr1, opr2) => match op {
                Mnemonic::Mov => self.gen_mov(opr1, opr2),
                _ => {
                    let opr1 = self.opr2opr(opr1);
                    let opr2 = self.opr2opr(opr2);
                    self.output.push(Instruction::new_binary(op, opr1, opr2));
                }
            },
        }
        Ok(())
    }

    fn gen_jmp(&mut self, op: Mnemonic, opr1: OperandNode) {
        if let OperandNode::Label { name } = opr1 {
            let cur = self.output.len();
            let label = self.lookup_label(name, cur);
            let jmp_code_len = encode::encode(&Instruction::new_unary(
                op.clone(),
                Operand::Offset(Offset::Off32(0)),
            ))
            .len();
            let offset = self.calc_offset(cur, label as usize) - jmp_code_len as i32;

            self.output.push(Instruction::new_unary(
                op,
                Operand::Offset(Offset::Off32(offset)),
            ));
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
                    if disp < 0x80 {
                        Displacement::Disp8(disp as i8)
                    } else {
                        Displacement::Disp32(disp as i32)
                    }
                }),
            )),
            OperandNode::Label { .. } => panic!(),
        }
    }

    fn lookup_label(&mut self, name: String, code_addr: usize) -> usize {
        match self.labels.get(&name) {
            Some(idx) => *idx,
            None => {
                self.unresolved_jumps.push((name, code_addr));
                0
            }
        }
    }

    fn resolve_jump(&mut self) -> Result<(), String> {
        for (name, cur) in &self.unresolved_jumps {
            match self.labels.get(name) {
                Some(label) => {
                    let op = self.output.get(*cur).unwrap().mnenomic.clone();
                    let jmp_code_len = encode::encode(&Instruction::new_unary(
                        op,
                        Operand::Offset(Offset::Off32(0)),
                    ))
                    .len();
                    let offset = self.calc_offset(*cur, *label) - jmp_code_len as i32;

                    let inst = self.output.get_mut(*cur).unwrap();
                    inst.operand1 = Some(Operand::Offset(Offset::Off32(offset)));
                }
                None => {
                    self.unknown_symbols.insert(name.clone());
                }
            }
        }
        Ok(())
    }
}
