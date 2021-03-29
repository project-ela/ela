use crate::{arch::x86::asm, ssa};

use super::InstructionSelector;

impl InstructionSelector {
    pub(crate) fn trans_inst(
        &mut self,
        inst_id: &ssa::InstructionId,
        inst: &ssa::Instruction,
    ) -> Vec<asm::Instruction> {
        use ssa::Instruction::*;

        match inst {
            BinOp(ssa::BinaryOperator::Add, lhs, rhs) => {
                vec![
                    asm::Instruction::new(
                        asm::Mnemonic::Mov,
                        vec![
                            asm::Operand::Register(asm::Register::Virtual(inst_id.index())),
                            self.trans_value(lhs),
                        ],
                    ),
                    asm::Instruction::new(
                        asm::Mnemonic::Add,
                        vec![
                            asm::Operand::Register(asm::Register::Virtual(inst_id.index())),
                            self.trans_value(rhs),
                        ],
                    ),
                ]
            }

            x => unimplemented!("{:?}", x),
        }
    }

    pub(crate) fn trans_term(&mut self, term: &ssa::Terminator) -> Vec<asm::Instruction> {
        use ssa::Terminator::*;

        match term {
            Ret(val) => {
                let mut inst = Vec::new();
                match val {
                    None => {}
                    Some(val) => {
                        inst.push(asm::Instruction::new(
                            asm::Mnemonic::Mov,
                            vec![
                                asm::Operand::Register(asm::Register::Physical(
                                    asm::MachineRegister::Rax,
                                )),
                                self.trans_value(val),
                            ],
                        ));
                    }
                }
                inst.push(asm::Instruction::new(
                    asm::Mnemonic::Jmp,
                    vec![asm::Operand::Label(self.return_label())],
                ));
                inst
            }
            Br(dst) => vec![asm::Instruction::new(
                asm::Mnemonic::Jmp,
                vec![asm::Operand::Label(self.block_label(*dst))],
            )],
            x => unimplemented!("{:?}", x),
        }
    }
}
