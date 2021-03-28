use id_arena::Arena;

use super::{Function, FunctionId, Global, GlobalId, Types};

#[derive(Debug)]
pub struct Module {
    pub functions: Arena<Function>,

    pub globals: Arena<Global>,

    pub types: Types,
}

impl Module {
    pub fn new() -> Self {
        Self {
            functions: Arena::new(),
            globals: Arena::new(),
            types: Types::new(),
        }
    }

    pub fn add_function(&mut self, function: Function) -> FunctionId {
        self.functions.alloc(function)
    }

    pub fn function(&self, func_id: FunctionId) -> Option<&Function> {
        self.functions.get(func_id)
    }

    pub fn add_global(&mut self, global: Global) -> GlobalId {
        self.globals.alloc(global)
    }

    pub fn global(&self, global_id: GlobalId) -> Option<&Global> {
        self.globals.get(global_id)
    }
}
