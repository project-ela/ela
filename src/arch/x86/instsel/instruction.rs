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
            BinOp(op, lhs, rhs) => self.trans_binop(inst_id, op, lhs, rhs),
            Cmp(op, lhs, rhs) => self.trans_cmp(inst_id, op, lhs, rhs),

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
            Param(_) => vec![],

            // do nothing
            Alloc(_) => vec![],
            Load(src) => vec![asm::Instruction::new(
                asm::Mnemonic::Mov,
                vec![
                    asm::Operand::Register(inst_id.into()),
                    self.trans_lvalue(module, src),
                ],
            )],
            Store(dst, src) => vec![asm::Instruction::new(
                asm::Mnemonic::Mov,
                vec![self.trans_lvalue(module, dst), self.trans_value(src)],
            )],
        }
    }

    fn trans_binop(
        &mut self,
        inst_id: &ssa::InstructionId,
        op: &ssa::BinaryOperator,
        lhs: &ssa::Value,
        rhs: &ssa::Value,
    ) -> Vec<asm::Instruction> {
        use ssa::BinaryOperator::*;

        let reg = asm::Operand::Register(inst_id.into());
        let lhs = self.trans_value(lhs);
        let rhs = self.trans_value(rhs);

        match op {
            Add => vec![
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), lhs]),
                asm::Instruction::new(asm::Mnemonic::Add, vec![reg, rhs]),
            ],
            Sub => vec![
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), lhs]),
                asm::Instruction::new(asm::Mnemonic::Sub, vec![reg, rhs]),
            ],
            Mul => vec![
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), lhs]),
                asm::Instruction::new(asm::Mnemonic::Imul, vec![reg, rhs]),
            ],
            Div => vec![
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        asm::Operand::Register(asm::MachineRegister::Rax.into()),
                        lhs,
                    ],
                ),
                asm::Instruction::new(asm::Mnemonic::Cqo, vec![]),
                asm::Instruction::new(asm::Mnemonic::Idiv, vec![rhs]),
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        reg,
                        asm::Operand::Register(asm::MachineRegister::Rax.into()),
                    ],
                ),
            ],
            Rem => vec![
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        asm::Operand::Register(asm::MachineRegister::Rax.into()),
                        lhs,
                    ],
                ),
                asm::Instruction::new(asm::Mnemonic::Cqo, vec![]),
                asm::Instruction::new(asm::Mnemonic::Idiv, vec![rhs]),
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        reg,
                        asm::Operand::Register(asm::MachineRegister::Rdx.into()),
                    ],
                ),
            ],

            Shl => vec![
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), lhs]),
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        asm::Operand::Register(asm::MachineRegister::Rcx.into()),
                        rhs,
                    ],
                ),
                asm::Instruction::new(
                    asm::Mnemonic::Shl,
                    vec![
                        reg,
                        asm::Operand::Register(asm::MachineRegister::Rcx.into()),
                    ],
                ),
            ],
            Shr => vec![
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), lhs]),
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        asm::Operand::Register(asm::MachineRegister::Rcx.into()),
                        rhs,
                    ],
                ),
                asm::Instruction::new(
                    asm::Mnemonic::Shr,
                    vec![
                        reg,
                        asm::Operand::Register(asm::MachineRegister::Rcx.into()),
                    ],
                ),
            ],

            And => vec![
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), lhs]),
                asm::Instruction::new(asm::Mnemonic::And, vec![reg, rhs]),
            ],
            Or => vec![
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), lhs]),
                asm::Instruction::new(asm::Mnemonic::Or, vec![reg, rhs]),
            ],
            Xor => vec![
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), lhs]),
                asm::Instruction::new(asm::Mnemonic::Xor, vec![reg, rhs]),
            ],
        }
    }

    fn trans_cmp(
        &mut self,
        inst_id: &ssa::InstructionId,
        op: &ssa::ComparisonOperator,
        lhs: &ssa::Value,
        rhs: &ssa::Value,
    ) -> Vec<asm::Instruction> {
        use ssa::ComparisonOperator::*;

        let reg = asm::Operand::Register(inst_id.into());
        let lhs = self.trans_value(lhs);
        let rhs = self.trans_value(rhs);

        let mut inst = Vec::new();
        inst.push(asm::Instruction::new(
            asm::Mnemonic::Mov,
            vec![reg.clone(), lhs],
        ));
        inst.push(asm::Instruction::new(
            asm::Mnemonic::Cmp,
            vec![reg.clone(), rhs],
        ));

        let mnemonic = match op {
            Eq => asm::Mnemonic::Sete,
            Neq => asm::Mnemonic::Setne,

            Gt => asm::Mnemonic::Setg,
            Gte => asm::Mnemonic::Setge,
            Lt => asm::Mnemonic::Setl,
            Lte => asm::Mnemonic::Setle,
        };
        inst.push(asm::Instruction::new(mnemonic, vec![reg.clone()]));

        inst.push(asm::Instruction::new(
            asm::Mnemonic::And,
            vec![reg, asm::Operand::Immediate(asm::Immediate::I8(1))],
        ));

        inst
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

    fn trans_lvalue(&mut self, module: &ssa::Module, val: &ssa::Value) -> asm::Operand {
        use ssa::Value::*;

        match val {
            Instruction(inst_val) => self.stack_offlsets.get(&inst_val.inst_id).unwrap().clone(),
            Global(ssa::GlobalValue { global_id, .. }) => {
                let global = module.global(*global_id).unwrap();
                asm::Operand::Indirect(asm::Indirect::new_label(
                    asm::MachineRegister::Rip.into(),
                    global.name.clone(),
                ))
            }
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
