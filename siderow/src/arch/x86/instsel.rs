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

    stack_offsets: HashMap<ssa::InstructionId, asm::Operand>,
    cur_func_name: String,

    geps: HashMap<ssa::InstructionId, asm::Operand>,
}

impl InstructionSelector {
    fn new() -> Self {
        Self {
            assembly: asm::Assembly::new(),
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
            if function.block_order.is_empty() {
                continue;
            }
            self.trans_function(&module, function);
        }

        self.assembly
    }

    // TODO
    fn trans_global(&mut self, global: &ssa::Global) {
        self.assembly.data.add_data(global.name.clone(), 8);
    }

    fn trans_function(&mut self, module: &ssa::Module, ssa_func: &ssa::Function) {
        let mut asm_func = asm::Function::new(&ssa_func.name);
        self.cur_func_name = ssa_func.name.clone();

        // prologue
        asm_func.add_inst(asm::Instruction::new(
            asm::Mnemonic::Push,
            vec![asm::Operand::Register(asm::MachineRegisterKind::Rbp.into())],
        ));
        asm_func.add_inst(asm::Instruction::new(
            asm::Mnemonic::Mov,
            vec![
                asm::Operand::Register(asm::MachineRegisterKind::Rbp.into()),
                asm::Operand::Register(asm::MachineRegisterKind::Rsp.into()),
            ],
        ));

        let stack_offset = self.calc_stack_offset(ssa_func);
        asm_func.add_inst(asm::Instruction::new(
            asm::Mnemonic::Sub,
            vec![
                asm::Operand::Register(asm::MachineRegisterKind::Rsp.into()),
                asm::Operand::Immediate(asm::Immediate::I32(stack_offset)),
            ],
        ));

        for reg in &asm::REGS {
            asm_func.add_inst(asm::Instruction::new(
                asm::Mnemonic::Push,
                vec![asm::Operand::Register(reg.clone().into())],
            ));
        }

        for block_id in &ssa_func.block_order {
            let block = ssa_func.block(*block_id).unwrap();
            asm_func.add_label(self.block_label(*block_id));

            self.trans_block(module, &ssa_func, block, &mut asm_func);
        }

        // epilogue
        asm_func.add_label(self.return_label());
        for reg in asm::REGS.iter().rev() {
            asm_func.add_inst(asm::Instruction::new(
                asm::Mnemonic::Pop,
                vec![asm::Operand::Register(reg.clone().into())],
            ));
        }
        asm_func.add_inst(asm::Instruction::new(
            asm::Mnemonic::Mov,
            vec![
                asm::Operand::Register(asm::MachineRegisterKind::Rsp.into()),
                asm::Operand::Register(asm::MachineRegisterKind::Rbp.into()),
            ],
        ));
        asm_func.add_inst(asm::Instruction::new(
            asm::Mnemonic::Pop,
            vec![asm::Operand::Register(asm::MachineRegisterKind::Rbp.into())],
        ));
        asm_func.add_inst(asm::Instruction::new(asm::Mnemonic::Ret, vec![]));

        self.assembly.text.add_function(asm_func);
    }

    fn calc_stack_offset(&mut self, function: &ssa::Function) -> i32 {
        // TODO
        let mut stack_offset = 0;
        self.stack_offsets.clear();
        for block_id in &function.block_order {
            let block = function.block(*block_id).unwrap();
            for inst_id in &block.instructions {
                let inst = function.inst(*inst_id).unwrap();

                if let ssa::InstructionKind::Alloc(ref typ) = inst.kind {
                    let align = typ.reg_size().size() as i32;
                    let typ_size = typ.size() as i32;
                    stack_offset = Self::align_to(stack_offset, align) + typ_size;

                    self.stack_offsets.insert(
                        *inst_id,
                        asm::Operand::Indirect(asm::Indirect::new_imm(
                            asm::MachineRegisterKind::Rbp.into(),
                            -stack_offset,
                            typ.reg_size(),
                        )),
                    );
                }
            }
        }

        stack_offset
    }

    fn align_to(x: i32, align: i32) -> i32 {
        (x + align - 1) & !(align - 1)
    }

    fn trans_block(
        &mut self,
        module: &ssa::Module,
        ssa_func: &ssa::Function,
        block: &ssa::Block,
        asm_func: &mut asm::Function,
    ) {
        for inst_id in &block.instructions {
            let ssa_inst = ssa_func.inst(*inst_id).unwrap();
            let asm_inst = self.trans_inst(module, inst_id, &ssa_inst.kind);
            for inst in asm_inst {
                asm_func.add_inst(inst);
            }
        }

        let term_id = match block.terminator {
            Some(term_id) => term_id,
            None => return,
        };
        let ssa_inst = ssa_func.inst(term_id).unwrap();
        let asm_inst = self.trans_term(term_id, &ssa_inst.kind);
        for inst in asm_inst {
            asm_func.add_inst(inst);
        }
    }

    fn block_label(&self, block_id: ssa::BlockId) -> String {
        format!(".{}.{}", self.cur_func_name, block_id.index())
    }

    fn return_label(&self) -> String {
        format!(".{}.ret", self.cur_func_name)
    }
}
