mod instruction;

use std::collections::HashMap;

use crate::ssa;

use super::asm;

pub fn translate(module: ssa::Module) -> asm::Assembly {
    let selector = InstructionSelector::new();
    selector.translate(module)
}

struct InstructionSelector {
    assembly: asm::Assembly,
    func: asm::Function,

    stack_offsets: HashMap<ssa::InstructionId, asm::Operand>,
    cur_func_name: String,

    geps: HashMap<ssa::InstructionId, asm::Operand>,
}

impl InstructionSelector {
    fn new() -> Self {
        Self {
            assembly: asm::Assembly::new(),
            func: asm::Function::new(),
            stack_offsets: HashMap::new(),
            cur_func_name: "".into(),
            geps: HashMap::new(),
        }
    }

    fn translate(mut self, module: ssa::Module) -> asm::Assembly {
        for (_, global) in &module.globals {
            self.trans_global(global);
        }

        for (_, function) in &module.functions {
            self.trans_function(&module, function);
        }

        self.assembly
    }

    // TODO
    fn trans_global(&mut self, global: &ssa::Global) {
        self.assembly.data.add_data(global.name.clone(), 8);
    }

    fn trans_function(&mut self, module: &ssa::Module, function: &ssa::Function) {
        self.cur_func_name = function.name.clone();
        self.func
            .add_pseudo_op(asm::PseudoOp::Global(self.cur_func_name.clone()));
        self.func.add_label(self.cur_func_name.clone());

        // prologue
        self.func.add_inst(asm::Instruction::new(
            asm::Mnemonic::Push,
            vec![asm::Operand::Register(asm::MachineRegister::Rbp.into())],
        ));
        self.func.add_inst(asm::Instruction::new(
            asm::Mnemonic::Mov,
            vec![
                asm::Operand::Register(asm::MachineRegister::Rbp.into()),
                asm::Operand::Register(asm::MachineRegister::Rsp.into()),
            ],
        ));

        let stack_offset = self.calc_stack_offset(function);
        self.func.add_inst(asm::Instruction::new(
            asm::Mnemonic::Sub,
            vec![
                asm::Operand::Register(asm::MachineRegister::Rsp.into()),
                asm::Operand::Immediate(asm::Immediate::I32(stack_offset)),
            ],
        ));

        for reg in &asm::REGS {
            self.func.add_inst(asm::Instruction::new(
                asm::Mnemonic::Push,
                vec![asm::Operand::Register(reg.clone().into())],
            ));
        }

        for block_id in &function.block_order {
            let block = function.block(*block_id).unwrap();
            self.func.add_label(self.block_label(*block_id));

            self.trans_block(module, &function, block);
        }

        // epilogue
        self.func.add_label(self.return_label());
        for reg in asm::REGS.iter().rev() {
            self.func.add_inst(asm::Instruction::new(
                asm::Mnemonic::Pop,
                vec![asm::Operand::Register(reg.clone().into())],
            ));
        }
        self.func.add_inst(asm::Instruction::new(
            asm::Mnemonic::Mov,
            vec![
                asm::Operand::Register(asm::MachineRegister::Rsp.into()),
                asm::Operand::Register(asm::MachineRegister::Rbp.into()),
            ],
        ));
        self.func.add_inst(asm::Instruction::new(
            asm::Mnemonic::Pop,
            vec![asm::Operand::Register(asm::MachineRegister::Rbp.into())],
        ));
        self.func
            .add_inst(asm::Instruction::new(asm::Mnemonic::Ret, vec![]));

        let mut new_func = asm::Function::new();
        std::mem::swap(&mut self.func, &mut new_func);
        self.assembly.text.add_function(new_func);
    }

    fn calc_stack_offset(&mut self, function: &ssa::Function) -> i32 {
        // TODO
        let mut stack_offset = 0;
        self.stack_offsets.clear();
        for block_id in &function.block_order {
            let block = function.block(*block_id).unwrap();
            for inst_id in &block.instructions {
                let inst = function.inst(*inst_id).unwrap();

                if let ssa::InstructionKind::Alloc(typ) = inst.kind {
                    stack_offset += typ.size(&function.types.borrow()) as i32;
                    self.stack_offsets.insert(
                        *inst_id,
                        asm::Operand::Indirect(asm::Indirect::new_imm(
                            asm::MachineRegister::Rbp.into(),
                            -stack_offset,
                        )),
                    );
                }
            }
        }

        stack_offset
    }

    fn trans_block(&mut self, module: &ssa::Module, function: &ssa::Function, block: &ssa::Block) {
        for inst_id in &block.instructions {
            let ssa_inst = function.inst(*inst_id).unwrap();
            let asm_inst = self.trans_inst(module, inst_id, &ssa_inst.kind);
            for inst in asm_inst {
                self.func.add_inst(inst);
            }
        }

        let term_id = match block.terminator {
            Some(term_id) => term_id,
            None => return,
        };
        let ssa_inst = function.inst(term_id).unwrap();
        let asm_inst = self.trans_term(term_id, &ssa_inst.kind);
        for inst in asm_inst {
            self.func.add_inst(inst);
        }
    }

    fn block_label(&self, block_id: ssa::BlockId) -> String {
        format!(".{}.{}", self.cur_func_name, block_id.index())
    }

    fn return_label(&self) -> String {
        format!(".{}.ret", self.cur_func_name)
    }
}
