use crate::{
    common::{
        error::{Error, ErrorKind},
        pos::Pos,
    },
    middleend::irgen::ir::*,
};
use std::collections::HashMap;

const REGS: [Register; 4] = [Register::R12, Register::R13, Register::R14, Register::R15];

struct RegAlloc {
    reg_map: HashMap<u32, Register>,
}

pub fn alloc_register(program: IRProgram) -> Result<IRProgram, Error> {
    let mut regalloc = RegAlloc::new();
    Ok(regalloc.alloc_register(program)?)
}

impl RegAlloc {
    fn new() -> Self {
        Self {
            reg_map: HashMap::new(),
        }
    }

    fn alloc_register(&mut self, mut program: IRProgram) -> Result<IRProgram, Error> {
        for function in program.functions.iter_mut() {
            for block in function.blocks.iter_mut() {
                self.alloc_register_block(block)?;
            }
            self.reg_map.clear();
        }
        Ok(program)
    }

    fn alloc_register_block(&mut self, block: &mut IRBlock) -> Result<(), Error> {
        for ir in block.irs.iter_mut() {
            match ir {
                IR::UnOp { op: _, src } => {
                    self.get_operand(src, false);
                }
                IR::BinOp {
                    op: _,
                    ref mut dst,
                    ref mut lhs,
                    ref mut rhs,
                } => {
                    self.get_operand(lhs, true);
                    self.get_operand(rhs, true);
                    self.alloc_operand(dst)?;
                }
                IR::Call { dst, name: _, args } => {
                    for arg in args {
                        self.get_operand(arg, true);
                    }
                    if let Some(dst) = dst {
                        self.alloc_operand(dst)?;
                    }
                }
                IR::Move { dst, src } => {
                    self.get_operand(src, true);
                    self.alloc_operand(dst)?;
                }
                IR::Addr { dst, src: _ } => {
                    self.alloc_operand(dst)?;
                }
                IR::Load { dst, src } => {
                    self.get_operand(src, true);
                    self.alloc_operand(dst)?;
                }
                IR::Store { dst, src } => {
                    self.get_operand(src, true);
                    self.get_operand(dst, true);
                }
                IR::StoreArg { dst: _, src: _ } => {}
                IR::Jump { .. } => {}
                IR::JumpIfNot { label: _, cond } => {
                    self.get_operand(cond, true);
                }
                IR::Ret { src } => {
                    if let Some(src) = src {
                        self.get_operand(src, true);
                    }
                }
            }
        }
        Ok(())
    }

    fn alloc_operand(&mut self, operand: &mut Operand) -> Result<(), Error> {
        match operand {
            Operand::Reg(ref mut info) => {
                info.physical_index = Some(self.alloc_reg(info.virtual_index)?);
            }
            Operand::Const(_) => {}
        }

        Ok(())
    }

    fn get_operand(&mut self, operand: &mut Operand, kill: bool) {
        match operand {
            Operand::Reg(ref mut info) => {
                info.physical_index = Some(self.get_reg(info.virtual_index));
            }
            Operand::Const(_) => {}
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
            Operand::Const(_) => {}
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
