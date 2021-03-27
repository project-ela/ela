use id_arena::Arena;

use super::{Function, FunctionId};

#[derive(Debug)]
pub struct Module {
    pub functions: Arena<Function>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            functions: Arena::new(),
        }
    }

    pub fn add_function(&mut self, function: Function) -> FunctionId {
        self.functions.alloc(function)
    }
}
