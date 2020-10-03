use crate::{
    common::{
        error::{Error, ErrorKind},
        pos::Pos,
    },
    middleend::tacgen::tac::*,
};
use std::collections::HashMap;

const REGS: [Register; 4] = [Register::Eax, Register::Ecx, Register::Edx, Register::Ebx];

struct RegAlloc {
    reg_map: HashMap<u32, Register>,
}

pub fn alloc_register(program: TacProgram) -> Result<TacProgram, Error> {
    let mut regalloc = RegAlloc::new();
    Ok(regalloc.alloc_register(program)?)
}

impl RegAlloc {
    fn new() -> Self {
        Self {
            reg_map: HashMap::new(),
        }
    }

    fn alloc_register(&mut self, mut program: TacProgram) -> Result<TacProgram, Error> {
        for function in program.functions.iter_mut() {
            for tac in function.body.iter_mut() {
                match tac {
                    Tac::Label { .. } => {}
                    Tac::UnOp { op: _, ref mut src } => {
                        self.get_operand(src, false);
                    }
                    Tac::BinOp {
                        op: _,
                        ref mut dst,
                        ref mut lhs,
                        ref mut rhs,
                    } => {
                        self.get_operand(lhs, true);
                        self.get_operand(rhs, true);
                        self.alloc_operand(dst)?;
                    }
                    Tac::Call { dst, .. } => {
                        if let Some(dst) = dst {
                            self.alloc_operand(dst)?;
                        }
                    }
                    Tac::Move { dst, src } => {
                        self.alloc_operand(dst)?;
                        self.get_operand(src, true);
                    }
                    Tac::Jump { .. } => {}
                    Tac::JumpIfNot {
                        label_index: _,
                        cond,
                    } => {
                        self.get_operand(cond, true);
                    }
                    Tac::Ret { src } => {
                        if let Some(src) = src {
                            self.get_operand(src, true);
                        }
                    }
                }
            }
            self.reg_map.clear();
        }
        Ok(program)
    }

    fn alloc_operand(&mut self, operand: &mut Operand) -> Result<(), Error> {
        match operand {
            Operand::Reg(ref mut info) => {
                info.physical_index = Some(self.alloc_reg(info.virtual_index)?);
            }
            Operand::Const(_) | Operand::Variable(_) => {}
        }

        Ok(())
    }

    fn get_operand(&mut self, operand: &mut Operand, kill: bool) {
        match operand {
            Operand::Reg(ref mut info) => {
                info.physical_index = Some(self.get_reg(info.virtual_index));
            }
            Operand::Const(_) | Operand::Variable(_) => {}
        }

        if kill {
            self.kill_operand(operand);
        }
    }

    fn kill_operand(&mut self, operand: &Operand) {
        match operand {
            Operand::Reg(info) => {
                self.kill_reg(&info.virtual_index);
            }
            Operand::Const(_) | Operand::Variable(_) => {}
        }
    }

    fn alloc_reg(&mut self, virtual_index: u32) -> Result<Register, Error> {
        for reg in &REGS {
            if self.reg_map.values().any(|val| val == reg) {
                continue;
            }
            self.reg_map.insert(virtual_index, *reg);
            return Ok(*reg);
        }

        Err(Error::new(Pos::default(), ErrorKind::RegistersExhausted))
    }

    fn get_reg(&mut self, virtual_index: u32) -> Register {
        *self.reg_map.get(&virtual_index).unwrap()
    }

    fn kill_reg(&mut self, virtual_index: &u32) {
        self.reg_map.remove(virtual_index);
    }
}
