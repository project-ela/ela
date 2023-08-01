use crate::ssa;

use super::asm;

pub fn translate(module: ssa::Module) -> asm::Assembly {
    asm::Assembly::new()
}
