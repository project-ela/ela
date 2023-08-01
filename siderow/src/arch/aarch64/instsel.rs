mod function;

use crate::ssa;

use self::function::FunctionTransrator;

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

            let translator = FunctionTransrator::new(&module, function);
            let asm_func = translator.translate();
            self.assembly.text.add_function(asm_func);
        }

        self.assembly
    }
}
