use std::collections::{HashMap, HashSet};

use super::asm::{Assembly, AssemblyItem, Function, MachineRegister, Operand, Register, REGS};

pub fn allocate(assembly: &mut Assembly) {
    let mut allocator = DummyRegisterAllocator::new();
    allocator.allocate(assembly);
}

struct DummyRegisterAllocator {
    reg_map: HashMap<usize, MachineRegister>,
}

// TODO
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
        let lifetimes = self.calc_lifetime(function);

        for (i, item) in function.items.iter_mut().enumerate() {
            let inst = match item {
                AssemblyItem::Instruction(ref mut inst) => inst,
                _ => continue,
            };

            for j in 0..inst.operands.len() {
                match inst.operands.get_mut(j).unwrap() {
                    Operand::Register(Register::Virtual(id)) => {
                        let next_reg = self.find_next_reg();
                        let reg = self.reg_map.entry(*id).or_insert(next_reg).clone();

                        inst.operands[j] = Operand::Register(reg.into());
                    }
                    _ => {}
                }
            }

            if lifetimes.contains_key(&i) {
                let id = lifetimes.get(&i).unwrap();
                self.reg_map.retain(|reg_id, _| !id.contains(reg_id));
            }
        }
    }

    // item_idx: Vec<virt_reg>
    fn calc_lifetime(&mut self, function: &Function) -> HashMap<usize, HashSet<usize>> {
        let mut lifetimes = HashMap::new();
        let mut current_regs = HashSet::new();

        for (i, item) in function.items.iter().enumerate().rev() {
            let inst = match item {
                AssemblyItem::Instruction(ref inst) => inst,
                _ => continue,
            };

            for j in 0..inst.operands.len() {
                let id = match inst.operands.get(j).unwrap() {
                    Operand::Register(Register::Virtual(id)) => id,
                    _ => continue,
                };
                if current_regs.contains(id) {
                    continue;
                }

                lifetimes.entry(i).or_insert(HashSet::new()).insert(*id);
                current_regs.insert(*id);
            }
        }

        lifetimes
    }

    fn find_next_reg(&self) -> MachineRegister {
        let mut regs = REGS.iter().cloned().collect::<HashSet<MachineRegister>>();

        for reg in self.reg_map.values() {
            regs.remove(reg);
        }

        regs.into_iter().next().unwrap()
    }
}
