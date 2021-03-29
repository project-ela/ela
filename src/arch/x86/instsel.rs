use crate::ssa::{self, InstructionId};

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
            vec![asm::Operand::Register(asm::Register::Physical(
                asm::MachineRegister::Rbp,
            ))],
        ));
        self.assembly.add_inst(asm::Instruction::new(
            asm::Mnemonic::Mov,
            vec![
                asm::Operand::Register(asm::Register::Physical(asm::MachineRegister::Rbp)),
                asm::Operand::Register(asm::Register::Physical(asm::MachineRegister::Rsp)),
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
                asm::Operand::Register(asm::Register::Physical(asm::MachineRegister::Rsp)),
                asm::Operand::Register(asm::Register::Physical(asm::MachineRegister::Rbp)),
            ],
        ));
        self.assembly.add_inst(asm::Instruction::new(
            asm::Mnemonic::Pop,
            vec![asm::Operand::Register(asm::Register::Physical(
                asm::MachineRegister::Rbp,
            ))],
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

    fn trans_inst(
        &mut self,
        inst_id: &InstructionId,
        inst: &ssa::Instruction,
    ) -> Vec<asm::Instruction> {
        use ssa::Instruction::*;

        match inst {
            BinOp(ssa::BinaryOperator::Add, lhs, rhs) => {
                vec![
                    asm::Instruction::new(
                        asm::Mnemonic::Mov,
                        vec![
                            asm::Operand::Register(asm::Register::Virtual(inst_id.index())),
                            self.trans_value(lhs),
                        ],
                    ),
                    asm::Instruction::new(
                        asm::Mnemonic::Add,
                        vec![
                            asm::Operand::Register(asm::Register::Virtual(inst_id.index())),
                            self.trans_value(rhs),
                        ],
                    ),
                ]
            }

            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_term(&mut self, term: &ssa::Terminator) -> Vec<asm::Instruction> {
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
                                asm::Operand::Register(asm::Register::Physical(
                                    asm::MachineRegister::Rax,
                                )),
                                self.trans_value(val),
                            ],
                        ));
                    }
                }
                inst.push(asm::Instruction::new(
                    asm::Mnemonic::Jmp,
                    vec![asm::Operand::Label(self.cur_ret_label.clone())],
                ));
                inst
            }
            x => unimplemented!("{:?}", x),
        }
    }

    fn trans_value(&mut self, val: &ssa::Value) -> asm::Operand {
        use ssa::Value::*;

        match val {
            Constant(r#const) => asm::Operand::Immediate(r#const.into()),
            Instruction(inst_val) => {
                asm::Operand::Register(asm::Register::Virtual(inst_val.inst_id.index()))
            }
            x => unimplemented!("{:?}", x),
        }
    }
}
