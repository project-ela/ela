use crate::{arch::x86::asm, ssa};

use super::InstructionSelector;

const ARG_REGS: [asm::MachineRegister; 6] = [
    asm::MachineRegister::Rdi,
    asm::MachineRegister::Rsi,
    asm::MachineRegister::Rdx,
    asm::MachineRegister::Rcx,
    asm::MachineRegister::R8,
    asm::MachineRegister::R9,
];

impl InstructionSelector {
    pub(crate) fn trans_inst(
        &mut self,
        module: &ssa::Module,
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

            Call(func_id, args) => {
                let func = module.function(*func_id).unwrap();
                let mut inst = Vec::new();
                for (i, arg) in args.iter().enumerate() {
                    inst.push(asm::Instruction::new(
                        asm::Mnemonic::Mov,
                        vec![self.arg_reg(i), self.trans_value(arg)],
                    ))
                }

                inst.push(asm::Instruction::new(
                    asm::Mnemonic::Call,
                    vec![asm::Operand::Label(func.name.clone())],
                ));
                inst.push(asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        asm::Operand::Register(inst_id.into()),
                        asm::Operand::Register(asm::MachineRegister::Rax.into()),
                    ],
                ));
                inst
            }

            // do nothing
            Alloc(_) => vec![],
            Load(src) => vec![asm::Instruction::new(
                asm::Mnemonic::Mov,
                vec![
                    asm::Operand::Register(inst_id.into()),
                    self.trans_lvalue(src),
                ],
            )],
            Store(dst, src) => vec![asm::Instruction::new(
                asm::Mnemonic::Mov,
                vec![self.trans_lvalue(dst), self.trans_value(src)],
            )],

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
        }
    }

    fn trans_value(&mut self, val: &ssa::Value) -> asm::Operand {
        use ssa::Value::*;

        match val {
            Constant(r#const) => asm::Operand::Immediate(r#const.into()),
            Instruction(inst_val) => asm::Operand::Register(inst_val.inst_id.into()),
            Parameter(ssa::ParameterValue { index, .. }) => self.arg_reg(*index),
            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_lvalue(&mut self, val: &ssa::Value) -> asm::Operand {
        use ssa::Value::*;

        match val {
            Instruction(inst_val) => self.stack_offlsets.get(&inst_val.inst_id).unwrap().clone(),
            x => panic!("{:?}", x),
        }
    }

    fn arg_reg(&mut self, index: usize) -> asm::Operand {
        if index >= 6 {
            unimplemented!()
        }

        let reg = ARG_REGS.get(index).unwrap().clone();
        asm::Operand::Register(reg.into())
    }
}
