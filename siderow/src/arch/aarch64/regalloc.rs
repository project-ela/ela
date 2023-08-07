use std::collections::HashMap;

use super::asm::{Assembly, AssemblyItem, MachineRegisterKind, RegisterKind};

pub fn allocate(assembly: &mut Assembly) {
    let regs = [
        MachineRegisterKind::X8,
        MachineRegisterKind::X9,
        MachineRegisterKind::X10,
        MachineRegisterKind::X11,
        MachineRegisterKind::X12,
        MachineRegisterKind::X13,
        MachineRegisterKind::X14,
        MachineRegisterKind::X15,
    ];
    let mut cur_reg = 0;
    let mut reg_map: HashMap<usize, usize> = HashMap::new();

    for function in &mut assembly.text.functions {
        for block in &mut function.blocks {
            for item in &mut block.items {
                let AssemblyItem::Instruction(inst) = item else { continue;};
                for operand in &mut inst.operands {
                    for virt_reg in operand.virt_regs_mut() {
                        let RegisterKind::Virtual(virt_reg_id) = virt_reg.kind else { continue; };
                        let phys_reg_idx = reg_map.entry(virt_reg_id).or_insert_with(|| {
                            if cur_reg >= regs.len() {
                                unimplemented!();
                            }
                            let reg = cur_reg;
                            cur_reg = cur_reg + 1;
                            reg
                        });
                        virt_reg.kind = RegisterKind::Physical(regs[*phys_reg_idx].clone());
                    }
                }
            }
        }
    }
}
