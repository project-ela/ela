use std::collections::HashMap;

use super::asm::{
    Assembly, AssemblyItem, Function, Instruction, MachineRegister, Operand, Register, REGS,
};

pub fn allocate(assembly: &mut Assembly) {
    let mut allocator = DummyRegisterAllocator::new();
    allocator.allocate(assembly);
}

struct DummyRegisterAllocator {
    reg_map: HashMap<usize, MachineRegister>,
}

impl DummyRegisterAllocator {
    fn new() -> Self {
        Self {
            reg_map: HashMap::new(),
        }
    }

    fn allocate(&mut self, assembly: &mut Assembly) {
        for function in assembly.text.functions.iter_mut() {
            self.reg_map.clear();
            self.alloc_function(function);
        }
    }

    fn alloc_function(&mut self, function: &mut Function) {
        for item in function.items.iter_mut() {
            match item {
                AssemblyItem::Instruction(ref mut inst) => self.alloc_inst(inst),
                _ => {}
            }
        }
    }

    fn alloc_inst(&mut self, inst: &mut Instruction) {
        for i in 0..inst.operands.len() {
            match inst.operands.get_mut(i).unwrap() {
                Operand::Register(Register::Virtual(id)) => {
                    let next_reg_index = self.reg_map.len();
                    let reg = self
                        .reg_map
                        .entry(*id)
                        .or_insert(REGS.get(next_reg_index).unwrap().clone())
                        .clone();

                    inst.operands[i] = Operand::Register(Register::Physical(reg));
                }
                _ => {}
            }
        }
    }
}
