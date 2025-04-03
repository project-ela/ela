use crate::{arch::aarch64::asm, ssa};

macro_rules! operand {
    ((reg_virt $id:expr)) => {
        $crate::arch::aarch64::asm::Operand::Register(
            $crate::arch::aarch64::asm::Register::new_virtual($id),
        )
    };
    ((reg_phys $name:tt)) => {
        $crate::arch::aarch64::asm::Operand::Register(
            $crate::arch::aarch64::asm::Register::new_physical(
                $crate::arch::aarch64::asm::MachineRegisterKind::$name,
            ),
        )
    };
    ((label $name:expr)) => {
        $crate::arch::aarch64::asm::Operand::Label($name)
    };
    ((cond $name:tt)) => {
        $crate::arch::aarch64::asm::Operand::Condition($crate::arch::aarch64::asm::Condition::$name)
    };
    ((value $value:expr)) => {
        $value
    };
}

macro_rules! inst {
    ($inst:tt $($operand:tt)*) => {
        $crate::arch::aarch64::asm::Instruction::new(
            $crate::arch::aarch64::asm::Mnemonic::$inst,
            vec![$(operand!($operand)),*],
        )
    };
}

const ARG_REGS: [asm::MachineRegisterKind; 9] = [
    asm::MachineRegisterKind::X0,
    asm::MachineRegisterKind::X1,
    asm::MachineRegisterKind::X2,
    asm::MachineRegisterKind::X3,
    asm::MachineRegisterKind::X4,
    asm::MachineRegisterKind::X5,
    asm::MachineRegisterKind::X6,
    asm::MachineRegisterKind::X7,
    asm::MachineRegisterKind::X8,
];

pub struct FunctionTransrator<'a> {
    module: &'a ssa::Module,
    function: &'a ssa::Function,
    next_virtual_register_id: usize,
}

impl<'a> FunctionTransrator<'a> {
    pub fn new(module: &'a ssa::Module, function: &'a ssa::Function) -> Self {
        Self {
            module,
            function,
            next_virtual_register_id: function.num_insts(),
        }
    }

    pub fn translate(mut self) -> asm::Function {
        let mut asm_func = asm::Function::new(&self.function.name);

        for block_id in &self.function.block_order {
            let block = self.function.block(*block_id).unwrap();
            let asm_block = self.trans_block(block);
            asm_func.add_block(asm_block);
        }
        asm_func.add_block(self.trans_epilogue());

        asm_func
    }

    fn trans_epilogue(&mut self) -> asm::Block {
        let mut asm_block = asm::Block::new(self.return_label());
        asm_block.add_inst(inst!(Ret));
        asm_block
    }

    fn trans_block(&mut self, block: &ssa::Block) -> asm::Block {
        let mut asm_block = asm::Block::new(self.block_label(&block.id));

        for inst_id in &block.instructions {
            let ssa_inst = self.function.inst(*inst_id).unwrap();
            let asm_inst = self.trans_inst(ssa_inst);
            for inst in asm_inst {
                asm_block.add_inst(inst);
            }
        }

        let Some(term_id) = block.terminator else {
            return asm_block;
        };
        let ssa_inst = self.function.inst(term_id).unwrap();
        let asm_inst = self.trans_term(ssa_inst);
        for inst in asm_inst {
            asm_block.add_inst(inst);
        }

        asm_block
    }

    fn trans_inst(&mut self, inst: &ssa::Instruction) -> Vec<asm::Instruction> {
        use ssa::InstructionKind::*;

        match &inst.kind {
            BinOp(op, lhs, rhs) => self.trans_binop(inst.id, op, lhs, rhs),
            Cmp(op, lhs, rhs) => self.trans_cmp(inst.id, op, lhs, rhs),
            Call(func_id, args) => self.trans_call(inst.id, func_id, args),
            _ => unimplemented!(),
        }
    }

    fn trans_binop(
        &mut self,
        inst_id: ssa::InstructionId,
        op: &ssa::BinaryOperator,
        lhs: &ssa::Value,
        rhs: &ssa::Value,
    ) -> Vec<asm::Instruction> {
        use ssa::BinaryOperator::*;

        let reg: asm::Operand = inst_id.into();
        let lhs = self.trans_value(lhs);
        let rhs = self.trans_value(rhs);

        match op {
            Add => vec![
                inst!(Mov (value reg.clone()) (value lhs)),
                inst!(Add (value reg.clone()) (value reg) (value rhs)),
            ],
            Sub => vec![
                inst!(Mov (value reg.clone()) (value lhs)),
                inst!(Sub (value reg.clone()) (value reg) (value rhs)),
            ],
            Mul => {
                let lhs_reg = self.alloc_virtual_register();
                let rhs_reg = self.alloc_virtual_register();
                vec![
                    inst!(Mov (value lhs_reg.clone()) (value lhs)),
                    inst!(Mov (value rhs_reg.clone()) (value rhs)),
                    inst!(Mul (value reg) (value lhs_reg) (value rhs_reg)),
                ]
            }
            Div => {
                let lhs_reg = self.alloc_virtual_register();
                let rhs_reg = self.alloc_virtual_register();
                vec![
                    inst!(Mov (value lhs_reg.clone()) (value lhs)),
                    inst!(Mov (value rhs_reg.clone()) (value rhs)),
                    inst!(SDiv (value reg) (value lhs_reg) (value rhs_reg)),
                ]
            }
            Rem => {
                let lhs_reg = self.alloc_virtual_register();
                let rhs_reg = self.alloc_virtual_register();
                vec![
                    inst!(Mov (value lhs_reg.clone()) (value lhs)),
                    inst!(Mov (value rhs_reg.clone()) (value rhs)),
                    inst!(SDiv (value reg.clone()) (value lhs_reg.clone()) (value rhs_reg.clone())),
                    inst!(MSub (value reg.clone()) (value rhs_reg) (value reg) (value lhs_reg)),
                ]
            }
            Shl => {
                let lhs_reg = self.alloc_virtual_register();
                vec![
                    inst!(Mov (value lhs_reg.clone()) (value lhs)),
                    inst!(Lsl (value reg.clone()) (value lhs_reg) (value rhs)),
                ]
            }
            Shr => {
                let lhs_reg = self.alloc_virtual_register();
                vec![
                    inst!(Mov (value lhs_reg.clone()) (value lhs)),
                    inst!(Lsr (value reg.clone()) (value lhs_reg) (value rhs)),
                ]
            }

            And => vec![
                inst!(Mov (value reg.clone()) (value lhs)),
                inst!(And (value reg.clone()) (value reg.clone()) (value rhs)),
            ],
            Or => vec![inst!(Orr (value reg.clone()) (value lhs) (value rhs))],
            Xor => {
                let lhs_reg = self.alloc_virtual_register();
                vec![
                    inst!(Mov (value lhs_reg.clone()) (value lhs)),
                    inst!(Eor (value reg.clone()) (value lhs_reg) (value rhs)),
                ]
            }
        }
    }

    fn trans_cmp(
        &mut self,
        inst_id: ssa::InstructionId,
        op: &ssa::ComparisonOperator,
        lhs: &ssa::Value,
        rhs: &ssa::Value,
    ) -> Vec<asm::Instruction> {
        let reg: asm::Operand = inst_id.into();
        let op = asm::Operand::Condition(op.into());
        let lhs = self.trans_value(lhs);
        let rhs = self.trans_value(rhs);

        vec![
            inst!(Mov (value reg.clone()) (value lhs)),
            inst!(Cmp (value reg.clone()) (value rhs)),
            inst!(CSet (value reg) (value op)),
        ]
    }

    fn trans_call(
        &mut self,
        inst_id: ssa::InstructionId,
        func_id: &ssa::FunctionId,
        args: &Vec<ssa::Value>,
    ) -> Vec<asm::Instruction> {
        let func = self.module.function(*func_id).unwrap();
        let mut inst = Vec::new();

        for (i, arg) in args.iter().enumerate() {
            let arg_reg = self.arg_reg(i);
            inst.extend(self.trans_move_value(l))
        }

        inst
    }

    fn trans_term(&mut self, inst: &ssa::Instruction) -> Vec<asm::Instruction> {
        use ssa::InstructionKind::*;

        match &inst.kind {
            Ret(value) => {
                let mut inst = Vec::new();
                if let Some(value) = value {
                    inst.push(inst!(Mov (reg_phys X0) (value self.trans_value(value))))
                }
                inst.push(inst!(B (label self.return_label())));
                inst
            }
            _ => unimplemented!(),
        }
    }

    fn trans_value(&mut self, value: &ssa::Value) -> asm::Operand {
        use ssa::Value::*;

        match value {
            Constant(value) => asm::Operand::Immediate(value.into()),
            Instruction(inst_val) => inst_val.inst_id.into(),
            _ => unimplemented!(),
        }
    }

    fn block_label(&self, block_id: &ssa::BlockId) -> String {
        format!(".{}.{}", self.function.name, block_id.index())
    }

    fn return_label(&self) -> String {
        format!(".{}.ret", self.function.name)
    }

    fn alloc_virtual_register(&mut self) -> asm::Operand {
        let id = self.next_virtual_register_id;
        self.next_virtual_register_id += 1;
        asm::Operand::Register(asm::Register::new_virtual(id))
    }

    fn arg_reg(&mut self, index: usize) -> asm::Register {
        if index >= ARG_REGS.len() {
            unimplemented!()
        }

        let reg = ARG_REGS.get(index).unwrap().clone();
        reg.into()
    }
}
