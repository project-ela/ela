use crate::ssa;

use super::asm;

pub fn translate(module: ssa::Module) -> asm::Assembly {
    let selector = InstructionSelector::new();
    selector.translate(module)
}

struct InstructionSelector {
    assembly: asm::Assembly,
}

impl InstructionSelector {
    fn new() -> Self {
        Self {
            assembly: asm::Assembly::new(),
        }
    }

    fn translate(mut self, module: ssa::Module) -> asm::Assembly {
        for (_, function) in &module.functions {
            if function.block_order.is_empty() {
                continue;
            }

            self.trans_function(&module, function);
        }

        self.assembly
    }

    fn trans_function(&mut self, module: &ssa::Module, ssa_func: &ssa::Function) {
        let asm_func = asm::Function::new(&ssa_func.name);
        self.assembly.text.add_function(asm_func);
    }
}
