use id_arena::Id;

use super::InstructionId;

pub type BlockId = Id<Block>;

#[derive(Debug)]
pub struct Block {
    pub instructions: Vec<InstructionId>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: InstructionId) {
        self.instructions.push(instruction)
    }
}
