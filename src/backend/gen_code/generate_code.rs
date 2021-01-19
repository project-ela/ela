use std::collections::HashMap;

use x86asm::instruction::{
    mnemonic::Mnemonic,
    operand::{
        immediate::Immediate,
        memory::{Displacement, Memory},
        offset::Offset,
        Operand,
    },
    Instruction,
};

use crate::{
    backend::gen_code::{Code, CodeItem, Codes, SectionName, UnresolvedJump},
    frontend::parser::node::{InstructionNode, OperandNode, Program, PseudoOp},
};

pub struct CodeGen {
    codes: Codes,
    current_section: SectionName,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            codes: HashMap::new(),
            current_section: SectionName::Text,
        }
    }

    pub fn gen_program(mut self, program: Program) -> Codes {
        for inst in program.insts {
            self.gen_inst(inst);
        }

        self.codes
    }

    fn gen_inst(&mut self, inst: InstructionNode) {
        match inst {
            InstructionNode::NullaryOp(op) => {
                self.add_item(CodeItem::Inst(Instruction::new_nullary(op)));
            }
            InstructionNode::UnaryOp(op, opr1) => {
                let opr1 = self.opr2opr(opr1);
                self.add_item(CodeItem::Inst(Instruction::new_unary(op, opr1)));
            }
            InstructionNode::BinaryOp(op, opr1, opr2) => {
                if matches!(op, Mnemonic::Mov) {
                    self.gen_mov(opr1, opr2);
                    return;
                }

                let opr1 = self.opr2opr(opr1);
                let opr2 = self.opr2opr(opr2);
                self.add_item(CodeItem::Inst(Instruction::new_binary(op, opr1, opr2)));
            }
            InstructionNode::PseudoOp(op, arg) => match op {
                PseudoOp::Data => self.current_section = SectionName::Data,
                PseudoOp::Text => self.current_section = SectionName::Text,
                PseudoOp::Zero => self.gen_zero(*arg.as_integer()),
                _ => {}
            },
            _ => {}
        }
    }

    fn gen_mov(&mut self, opr1: OperandNode, opr2: OperandNode) {
        let opr1 = self.opr2opr(opr1);
        let mut opr2 = self.opr2opr(opr2);

        if let Operand::Immediate(Immediate::Imm8(value)) = opr2 {
            opr2 = Operand::Immediate(Immediate::Imm32(value as i32));
        }

        self.add_item(CodeItem::Inst(Instruction::new_binary(
            Mnemonic::Mov,
            opr1,
            opr2,
        )));
    }

    fn gen_zero(&mut self, arg: u32) {
        self.add_item(CodeItem::Raw(vec![0; arg as usize]));
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
            OperandNode::Label { name: label_name } => {
                let cur_section = self.cur_section();
                let item_index = cur_section.items.len();
                cur_section.unresolved_jumps.push(UnresolvedJump {
                    label_name,
                    item_index,
                });

                Operand::Offset(Offset::Off32(0))
            }
        }
    }

    fn add_item(&mut self, item: CodeItem) {
        self.cur_section().items.push(item);
    }

    fn cur_section(&mut self) -> &mut Code {
        self.codes
            .entry(self.current_section.clone())
            .or_insert_with(Code::default)
    }
}
