use crate::{arch::aarch64::asm, ssa};

pub struct FunctionTransrator<'a> {
    module: &'a ssa::Module,
    function: &'a ssa::Function,
}

impl<'a> FunctionTransrator<'a> {
    pub fn new(module: &'a ssa::Module, function: &'a ssa::Function) -> Self {
        Self { module, function }
    }

    pub fn translate(self) -> asm::Function {
        let mut asm_func = asm::Function::new(&self.function.name);

        for block_id in &self.function.block_order {
            asm_func.add_label(self.block_label(block_id));
        }

        asm_func.add_label(self.return_label());

        asm_func
    }

    fn block_label(&self, block_id: &ssa::BlockId) -> String {
        format!(".{}.{}", self.function.name, block_id.index())
    }

    fn return_label(&self) -> String {
        format!(".{}.ret", self.function.name)
    }
}
