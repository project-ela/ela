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

pub struct FunctionTransrator<'a> {
    module: &'a ssa::Module,
    function: &'a ssa::Function,
}

impl<'a> FunctionTransrator<'a> {
    pub fn new(module: &'a ssa::Module, function: &'a ssa::Function) -> Self {
        Self { module, function }
    }

    pub fn translate(mut self) -> asm::Function {
        let mut asm_func = asm::Function::new(&self.function.name);

        for block_id in &self.function.block_order {
            asm_func.add_label(self.block_label(block_id));
            let block = self.function.block(*block_id).unwrap();
            self.trans_block(block, &mut asm_func);
        }

        asm_func.add_label(self.return_label());
        asm_func.add_inst(inst!(Ret));

        asm_func
    }

    fn trans_block(&mut self, block: &ssa::Block, asm_func: &mut asm::Function) {
        for inst_id in &block.instructions {
            let ssa_inst = self.function.inst(*inst_id).unwrap();
            let asm_inst = self.trans_inst(ssa_inst);
            for inst in asm_inst {
                asm_func.add_inst(inst);
            }
        }

        let Some(term_id) = block.terminator else { return; };
        let ssa_inst = self.function.inst(term_id).unwrap();
        let asm_inst = self.trans_term(ssa_inst);
        for inst in asm_inst {
            asm_func.add_inst(inst);
        }
    }

    fn trans_inst(&mut self, inst: &ssa::Instruction) -> Vec<asm::Instruction> {
        use ssa::InstructionKind::*;

        match &inst.kind {
            BinOp(op, lhs, rhs) => self.trans_binop(inst.id, op, lhs, rhs),
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
            _ => unimplemented!(),
        }
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
}
