use crate::middleend::tacgen::tac::*;
use std::collections::HashMap;

const REGS: [Register; 4] = [Register::Eax, Register::Ecx, Register::Edx, Register::Ebx];

struct RegAlloc {
    reg_map: HashMap<u32, Register>,
}

pub fn alloc_register(program: TacProgram) -> Result<TacProgram, String> {
    let mut regalloc = RegAlloc::new();
    Ok(regalloc.alloc_register(program))
}

impl RegAlloc {
    fn new() -> Self {
        Self {
            reg_map: HashMap::new(),
        }
    }

    fn alloc_register(&mut self, mut program: TacProgram) -> TacProgram {
        for function in program.functions.iter_mut() {
            for tac in function.body.iter_mut() {
                match tac {
                    Tac::Label { .. } => {}
                    Tac::UnOp { op: _, ref mut src } => {
                        self.get_operand(src);
                    }
                    Tac::BinOp {
                        op: _,
                        ref mut dst,
                        ref mut lhs,
                        ref mut rhs,
                    } => {
                        self.get_operand(lhs);
                        self.kill_operand(lhs);
                        self.get_operand(rhs);
                        self.kill_operand(rhs);
                        self.alloc_operand(dst);
                    }
                    Tac::Call { dst, .. } => {
                        self.alloc_operand(dst);
                    }
                    Tac::Move { dst, src } => {
                        self.alloc_operand(dst);
                        self.get_operand(src);
                        self.kill_operand(src);
                    }
                    Tac::Jump { .. } => {}
                    Tac::JumpIfNot {
                        label_index: _,
                        cond,
                    } => {
                        self.get_operand(cond);
                        self.kill_operand(cond);
                    }
                    Tac::Ret { src } => {
                        self.get_operand(src);
                        self.kill_operand(src);
                    }
                }
            }
            self.reg_map.clear();
        }
        program
    }

    fn alloc_operand(&mut self, operand: &mut Operand) {
        match operand {
            Operand::Const(_) => {}
            Operand::Reg(ref mut info) => {
                info.physical_index = Some(self.alloc_reg(info.virtual_index));
            }
            Operand::Variable(_) => {}
        }
    }

    fn get_operand(&mut self, operand: &mut Operand) {
        match operand {
            Operand::Const(_) => {}
            Operand::Reg(ref mut info) => {
                info.physical_index = Some(self.get_reg(info.virtual_index));
            }
            Operand::Variable(_) => {}
        }
    }

    fn kill_operand(&mut self, operand: &Operand) {
        match operand {
            Operand::Const(_) => {}
            Operand::Reg(info) => {
                self.kill_reg(&info.virtual_index);
            }
            Operand::Variable(_) => {}
        }
    }

    fn alloc_reg(&mut self, virtual_index: u32) -> Register {
        for reg in &REGS {
            if self.reg_map.values().any(|val| val == reg) {
                continue;
            }
            self.reg_map.insert(virtual_index, *reg);
            return *reg;
        }

        panic!("Error: registers exhausted");
    }

    fn get_reg(&mut self, virtual_index: u32) -> Register {
        return *self.reg_map.get(&virtual_index).unwrap();
    }

    fn kill_reg(&mut self, virtual_index: &u32) {
        self.reg_map.remove(virtual_index);
    }
}
