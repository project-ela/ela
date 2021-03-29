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
                            asm::Operand::Register(inst_id.into()),
                            self.trans_value(lhs),
                        ],
                    ),
                    asm::Instruction::new(
                        asm::Mnemonic::Add,
                        vec![
                            asm::Operand::Register(inst_id.into()),
                            self.trans_value(rhs),
                        ],
                    ),
                ]
            }
            Cmp(ssa::ComparisonOperator::Eq, lhs, rhs) => {
                vec![
                    asm::Instruction::new(
                        asm::Mnemonic::Mov,
                        vec![
                            asm::Operand::Register(inst_id.into()),
                            self.trans_value(lhs),
                        ],
                    ),
                    asm::Instruction::new(
                        asm::Mnemonic::Cmp,
                        vec![
                            asm::Operand::Register(inst_id.into()),
                            self.trans_value(rhs),
                        ],
                    ),
                    asm::Instruction::new(
                        asm::Mnemonic::Sete,
                        vec![asm::Operand::Register(inst_id.into())],
                    ),
                    asm::Instruction::new(
                        asm::Mnemonic::And,
                        vec![
                            asm::Operand::Register(inst_id.into()),
                            asm::Operand::Immediate(asm::Immediate::I8(1)),
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
                                asm::Operand::Register(asm::MachineRegister::Rax.into()),
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
            CondBr(cond, con, alt) => vec![
                asm::Instruction::new(
                    asm::Mnemonic::Cmp,
                    vec![
                        asm::Operand::Immediate(asm::Immediate::I8(0)),
                        self.trans_value(cond),
                    ],
                ),
                asm::Instruction::new(
                    asm::Mnemonic::Je,
                    vec![asm::Operand::Label(self.block_label(*alt))],
                ),
                asm::Instruction::new(
                    asm::Mnemonic::Jmp,
                    vec![asm::Operand::Label(self.block_label(*con))],
                ),
            ],
            x => unimplemented!("{:?}", x),
        }
    }
}
