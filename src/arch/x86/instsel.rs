mod instruction;

use crate::ssa;

use super::asm;

pub fn translate(module: ssa::Module) -> asm::Assembly {
    let selector = InstructionSelector::new();
    selector.translate(module)
}

struct InstructionSelector {
    assembly: asm::Assembly,

    cur_func_name: String,
}

impl InstructionSelector {
    fn new() -> Self {
        Self {
            assembly: asm::Assembly::new(),
            cur_func_name: "".into(),
        }
    }

    fn translate(mut self, module: ssa::Module) -> asm::Assembly {
        for (_, function) in module.functions {
            self.trans_function(function);
        }

        self.assembly
    }

    fn trans_function(&mut self, function: ssa::Function) {
        self.cur_func_name = function.name.clone();
        self.assembly
            .add_pseudo_op(asm::PseudoOp::Global(self.cur_func_name.clone()));
        self.assembly.add_label(self.cur_func_name.clone());

        // prologue
        self.assembly.add_inst(asm::Instruction::new(
            asm::Mnemonic::Push,
            vec![asm::Operand::Register(asm::MachineRegister::Rbp.into())],
        ));
        self.assembly.add_inst(asm::Instruction::new(
            asm::Mnemonic::Mov,
            vec![
                asm::Operand::Register(asm::MachineRegister::Rbp.into()),
                asm::Operand::Register(asm::MachineRegister::Rsp.into()),
            ],
        ));

        for (block_id, block) in &function.blocks {
            self.assembly.add_label(self.block_label(block_id));

            self.trans_block(&function, block);
        }

        // epilogue
        self.assembly.add_label(self.return_label());
        self.assembly.add_inst(asm::Instruction::new(
            asm::Mnemonic::Mov,
            vec![
                asm::Operand::Register(asm::MachineRegister::Rsp.into()),
                asm::Operand::Register(asm::MachineRegister::Rbp.into()),
            ],
        ));
        self.assembly.add_inst(asm::Instruction::new(
            asm::Mnemonic::Pop,
            vec![asm::Operand::Register(asm::MachineRegister::Rbp.into())],
        ));
        self.assembly
            .add_inst(asm::Instruction::new(asm::Mnemonic::Ret, vec![]))
    }

    fn trans_block(&mut self, function: &ssa::Function, block: &ssa::Block) {
        for inst_id in &block.instructions {
            let ssa_inst = function.inst(*inst_id).unwrap();
            let asm_inst = self.trans_inst(inst_id, ssa_inst);
            for inst in asm_inst {
                self.assembly.add_inst(inst);
            }
        }

        let term_id = block.terminator.unwrap();
        let ssa_term = function.term(term_id).unwrap();
        let asm_inst = self.trans_term(ssa_term);
        for inst in asm_inst {
            self.assembly.add_inst(inst);
        }
    }

    fn trans_value(&mut self, val: &ssa::Value) -> asm::Operand {
        use ssa::Value::*;

        match val {
            Constant(r#const) => asm::Operand::Immediate(r#const.into()),
            Instruction(inst_val) => asm::Operand::Register(inst_val.inst_id.into()),
            x => unimplemented!("{:?}", x),
        }
    }

    fn block_label(&self, block_id: ssa::BlockId) -> String {
        format!(".{}.{}", self.cur_func_name, block_id.index())
    }

    fn return_label(&self) -> String {
        format!(".{}.ret", self.cur_func_name)
    }
}
