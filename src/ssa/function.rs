use id_arena::Arena;

use super::{Instruction, InstructionId};

#[derive(Debug)]
pub struct Function {
    pub name: String,

    pub instructions: Arena<Instruction>,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            instructions: Arena::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> InstructionId {
        self.instructions.alloc(instruction)
    }
}
