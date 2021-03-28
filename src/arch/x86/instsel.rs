use crate::ssa;

use super::asm;

pub fn translate(module: ssa::Module) -> asm::Assembly {
    let selector = InstructionSelector::new();
    selector.translate(module)
}

struct InstructionSelector {
    assembly: asm::Assembly,

    cur_ret_label: String,
}

impl InstructionSelector {
    fn new() -> Self {
        Self {
            assembly: asm::Assembly::new(),
            cur_ret_label: "".into(),
        }
    }

    fn translate(mut self, module: ssa::Module) -> asm::Assembly {
        for (_, function) in module.functions {
            self.trans_function(function);
        }

        self.assembly
    }

    fn trans_function(&mut self, function: ssa::Function) {
        let func_name = function.name.clone();
        self.assembly
            .add_pseudo_op(asm::PseudoOp::Global(func_name.clone()));
        self.assembly.add_label(func_name.clone());
        self.cur_ret_label = format!(".{}.ret", func_name);

        // prologue
        self.assembly.add_inst(asm::Instruction::new(
            asm::Mnemonic::Push,
            vec![asm::Operand::Register(asm::Register::Rbp)],
        ));
        self.assembly.add_inst(asm::Instruction::new(
            asm::Mnemonic::Mov,
            vec![
                asm::Operand::Register(asm::Register::Rbp),
                asm::Operand::Register(asm::Register::Rsp),
            ],
        ));

        for (i, block) in &function.blocks {
            let block_name = format!(".{}.{}", func_name.clone(), i.index());
            self.assembly.add_label(block_name);

            self.trans_block(&function, block);
        }

        // epilogue
        self.assembly.add_label(self.cur_ret_label.clone());
        self.assembly.add_inst(asm::Instruction::new(
            asm::Mnemonic::Mov,
            vec![
                asm::Operand::Register(asm::Register::Rsp),
                asm::Operand::Register(asm::Register::Rbp),
            ],
        ));
        self.assembly.add_inst(asm::Instruction::new(
            asm::Mnemonic::Pop,
            vec![asm::Operand::Register(asm::Register::Rbp)],
        ));
        self.assembly
            .add_inst(asm::Instruction::new(asm::Mnemonic::Ret, vec![]))
    }

    fn trans_block(&mut self, function: &ssa::Function, block: &ssa::Block) {
        for inst_id in &block.instructions {
            let inst = function.inst(*inst_id).unwrap();
            self.trans_inst(inst);
        }

        let term_id = block.terminator.unwrap();
        let term = function.term(term_id).unwrap();
        self.trans_term(term);
    }

    fn trans_inst(&mut self, inst: &ssa::Instruction) {
        use ssa::Instruction::*;

        match inst {
            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_term(&mut self, term: &ssa::Terminator) {
        use ssa::Terminator::*;

        match term {
            Ret(val) => {
                match val {
                    None => {}
                    Some(ssa::Value::Constant(r#const)) => {
                        self.assembly.add_inst(asm::Instruction::new(
                            asm::Mnemonic::Mov,
                            vec![
                                asm::Operand::Register(asm::Register::Rax),
                                asm::Operand::Immediate(r#const.into()),
                            ],
                        ))
                    }
                    _ => unimplemented!(),
                }
                self.assembly.add_inst(asm::Instruction::new(
                    asm::Mnemonic::Jmp,
                    vec![asm::Operand::Label(self.cur_ret_label.clone())],
                ))
            }
            x => unimplemented!("{:?}", x),
        }
    }
}
