use crate::{
    arch::x86::{
        asm,
        instsel::{self, layout},
    },
    ssa,
};

const ARG_REGS: [asm::MachineRegisterKind; 6] = [
    asm::MachineRegisterKind::Rdi,
    asm::MachineRegisterKind::Rsi,
    asm::MachineRegisterKind::Rdx,
    asm::MachineRegisterKind::Rcx,
    asm::MachineRegisterKind::R8,
    asm::MachineRegisterKind::R9,
];

impl instsel::InstructionSelector {
    pub(crate) fn trans_inst(
        &mut self,
        module: &ssa::Module,
        inst_id: &ssa::InstructionId,
        inst_kind: &ssa::InstructionKind,
    ) -> Vec<asm::Instruction> {
        use ssa::InstructionKind::*;

        match inst_kind {
            BinOp(op, lhs, rhs) => self.trans_binop(inst_id, op, lhs, rhs),
            Cmp(op, lhs, rhs) => self.trans_cmp(inst_id, op, lhs, rhs),

            Call(func_id, args) => {
                let func = module.function(*func_id).unwrap();
                let mut inst = Vec::new();
                for (i, arg) in args.iter().enumerate() {
                    let arg_reg = self.arg_reg(i);
                    inst.extend(self.trans_move_value(
                        module,
                        *inst_id,
                        asm::Operand::Register(arg_reg),
                        arg,
                    ));
                }

                inst.push(asm::Instruction::new(
                    asm::Mnemonic::Call,
                    vec![asm::Operand::Label(func.name.clone())],
                ));
                inst.push(asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        asm::Operand::Register(inst_id.into()),
                        asm::Operand::Register(asm::MachineRegisterKind::Rax.into()),
                    ],
                ));
                inst
            }
            // do nothing
            Param(_) => vec![],

            // do nothing
            Alloc(_) => vec![],
            Load(src) => vec![Self::trans_mov(
                asm::Operand::Register(inst_id.into()),
                self.trans_lvalue(module, src),
            )],
            Store(dst, src) => match src {
                ssa::Value::Constant(ssa::Constant::ZeroInitializer) => {
                    self.trans_zero_init(module, dst)
                }
                x => {
                    let dst = self.trans_lvalue(module, dst);
                    self.trans_move_value(module, *inst_id, dst, x)
                }
            },

            Gep(val, indices) => {
                let dst = self.trans_gep(module, val, indices);
                self.geps.insert(*inst_id, dst);
                vec![]
            }

            x => unreachable!("{:?}", x),
        }
    }

    fn trans_zero_init(&mut self, module: &ssa::Module, dst: &ssa::Value) -> Vec<asm::Instruction> {
        let asm_dst = self.trans_lvalue(module, dst);
        let elm_typ = dst.typ().elm_typ();
        match elm_typ {
            ssa::Type::I1 | ssa::Type::I8 => vec![asm::Instruction::new(
                asm::Mnemonic::Mov,
                vec![asm_dst, asm::Operand::Immediate(asm::Immediate::I8(0))],
            )],
            ssa::Type::I32 => vec![asm::Instruction::new(
                asm::Mnemonic::Mov,
                vec![asm_dst, asm::Operand::Immediate(asm::Immediate::I32(0))],
            )],
            ssa::Type::Array(_, len) => {
                let zero = ssa::Value::new_i32(0);

                let mut inst = Vec::new();
                for i in 0..len {
                    let indices = vec![zero.clone(), ssa::Value::new_i32(i as i32)];
                    let dst = self.trans_gep(module, dst, &indices);

                    inst.push(asm::Instruction::new(
                        asm::Mnemonic::Mov,
                        vec![dst, asm::Operand::Immediate(asm::Immediate::I32(0))],
                    ));
                }

                inst
            }
            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_move_value(
        &mut self,
        module: &ssa::Module,
        inst_id: ssa::InstructionId,
        dst: asm::Operand,
        src: &ssa::Value,
    ) -> Vec<asm::Instruction> {
        let is_address = match src {
            ssa::Value::Instruction(inst_val) => {
                self.geps.contains_key(&inst_val.inst_id)
                    || self.stack_offsets.contains_key(&inst_val.inst_id)
            }
            _ => false,
        };

        if is_address {
            let reg = asm::Operand::Register(inst_id.into());
            vec![
                asm::Instruction::new(
                    asm::Mnemonic::Lea,
                    vec![reg.clone(), self.trans_lvalue(module, src)],
                ),
                asm::Instruction::new(asm::Mnemonic::Mov, vec![dst, reg]),
            ]
        } else {
            vec![Self::trans_mov(dst, self.trans_value(src))]
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
                        asm::Operand::Register(asm::MachineRegisterKind::Rax.into()),
                        lhs,
                    ],
                ),
                asm::Instruction::new(asm::Mnemonic::Cqo, vec![]),
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), rhs]),
                asm::Instruction::new(asm::Mnemonic::Idiv, vec![reg.clone()]),
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        reg,
                        asm::Operand::Register(asm::MachineRegisterKind::Rax.into()),
                    ],
                ),
            ],
            Rem => vec![
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        asm::Operand::Register(asm::MachineRegisterKind::Rax.into()),
                        lhs,
                    ],
                ),
                asm::Instruction::new(asm::Mnemonic::Cqo, vec![]),
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), rhs]),
                asm::Instruction::new(asm::Mnemonic::Idiv, vec![reg.clone()]),
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        reg,
                        asm::Operand::Register(asm::MachineRegisterKind::Rdx.into()),
                    ],
                ),
            ],

            Shl => vec![
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), lhs]),
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        asm::Operand::Register(asm::MachineRegisterKind::Rcx.into()),
                        rhs,
                    ],
                ),
                asm::Instruction::new(
                    asm::Mnemonic::Shl,
                    vec![
                        reg,
                        asm::Operand::Register(asm::MachineRegisterKind::Cl.into()),
                    ],
                ),
            ],
            Shr => vec![
                asm::Instruction::new(asm::Mnemonic::Mov, vec![reg.clone(), lhs]),
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![
                        asm::Operand::Register(asm::MachineRegisterKind::Cl.into()),
                        rhs,
                    ],
                ),
                asm::Instruction::new(
                    asm::Mnemonic::Shr,
                    vec![
                        reg,
                        asm::Operand::Register(asm::MachineRegisterKind::Cl.into()),
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
        inst.push(asm::Instruction::new(
            mnemonic,
            vec![asm::Operand::Register(asm::MachineRegisterKind::Cl.into())],
        ));

        inst.push(asm::Instruction::new(
            asm::Mnemonic::Movzx,
            vec![
                reg,
                asm::Operand::Register(asm::MachineRegisterKind::Cl.into()),
            ],
        ));

        inst
    }

    fn trans_gep(
        &mut self,
        module: &ssa::Module,
        val: &ssa::Value,
        indices: &[ssa::Value],
    ) -> asm::Operand {
        let mut indirect = match self.trans_lvalue(module, val) {
            asm::Operand::Indirect(indirect) => indirect,
            x => unimplemented!("{:?}", x),
        };

        let ret_typ = ssa::gep_elm_typ(val, indices);
        indirect.size = layout::register_size(&ret_typ);

        let mut disp_offset = 0;
        for i in 0..indices.len() {
            match indices[i] {
                ssa::Value::Constant(ssa::Constant::I32(index)) => {
                    let val_typ = match i {
                        0 => val.typ(),
                        _ => ssa::gep_elm_typ(val, &indices[..i]),
                    };
                    disp_offset += layout::member_offset_in_bits(&val_typ, index as usize);
                }
                ssa::Value::Instruction(ref inst_val) => {
                    let index = inst_val.inst_id.into();
                    indirect.set_index(index);
                }
                ssa::Value::Parameter(ssa::ParameterValue { index, .. }) => {
                    indirect.set_index(self.arg_reg(index));
                }
                ref x => unimplemented!("{:?}", x),
            }
        }
        indirect.disp_offset += disp_offset as i32;

        asm::Operand::Indirect(indirect)
    }

    pub(crate) fn trans_term(
        &mut self,
        inst_id: ssa::InstructionId,
        inst_kind: &ssa::InstructionKind,
    ) -> Vec<asm::Instruction> {
        use ssa::InstructionKind::*;

        let reg = asm::Operand::Register(inst_id.into());

        match inst_kind {
            Ret(val) => {
                let mut inst = Vec::new();
                match val {
                    None => {}
                    Some(val) => {
                        inst.push(asm::Instruction::new(
                            asm::Mnemonic::Mov,
                            vec![
                                asm::Operand::Register(asm::MachineRegisterKind::Rax.into()),
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
                // TODO
                asm::Instruction::new(
                    asm::Mnemonic::Mov,
                    vec![reg.clone(), self.trans_value(cond)],
                ),
                asm::Instruction::new(
                    asm::Mnemonic::Cmp,
                    vec![reg, asm::Operand::Immediate(asm::Immediate::I8(0))],
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

            x => unreachable!("{:?}", x),
        }
    }

    fn trans_value(&mut self, val: &ssa::Value) -> asm::Operand {
        use ssa::Value::*;

        match val {
            Constant(r#const) => asm::Operand::Immediate(r#const.into()),
            Instruction(inst_val) => asm::Operand::Register(inst_val.inst_id.into()),
            Parameter(ssa::ParameterValue { index, .. }) => {
                asm::Operand::Register(self.arg_reg(*index))
            }
            x => panic!("{:?}", x),
        }
    }

    fn trans_lvalue(&mut self, module: &ssa::Module, val: &ssa::Value) -> asm::Operand {
        use ssa::Value::*;

        let elm_typ = val.typ().elm_typ();
        let reg_size = layout::register_size(&elm_typ);

        match val {
            Instruction(inst_val) => {
                if let Some(gep) = self.geps.get(&inst_val.inst_id) {
                    return gep.clone();
                }

                if let Some(offset) = self.stack_offsets.get(&inst_val.inst_id) {
                    return offset.clone();
                }

                let base = inst_val.inst_id.into();
                asm::Operand::Indirect(asm::Indirect::new_imm(base, 0, reg_size))
            }
            Global(ssa::GlobalValue { global_id, .. }) => {
                let global = module.global(*global_id).unwrap();
                asm::Operand::Indirect(asm::Indirect::new_label(
                    asm::MachineRegisterKind::Rip.into(),
                    global.name.clone(),
                    reg_size,
                ))
            }
            Parameter(param_val) => asm::Operand::Indirect(asm::Indirect::new_imm(
                self.arg_reg(param_val.index),
                0,
                reg_size,
            )),
            x => panic!("{:?}", x),
        }
    }

    fn trans_mov(dst: asm::Operand, mut src: asm::Operand) -> asm::Instruction {
        let mnemonic = match (dst.size(), src.size()) {
            (asm::RegisterSize::QWord, asm::RegisterSize::Byte) => asm::Mnemonic::Movzx,
            (asm::RegisterSize::Byte, asm::RegisterSize::QWord) => {
                if let asm::Operand::Register(ref mut reg) = src {
                    reg.set_size(asm::RegisterSize::Byte);
                }
                asm::Mnemonic::Mov
            }
            _ => asm::Mnemonic::Mov,
        };
        asm::Instruction::new(mnemonic, vec![dst, src])
    }

    fn arg_reg(&mut self, index: usize) -> asm::Register {
        if index >= ARG_REGS.len() {
            unimplemented!()
        }

        let reg = ARG_REGS.get(index).unwrap().clone();
        reg.into()
    }
}
