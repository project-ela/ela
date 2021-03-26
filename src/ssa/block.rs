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

    pub fn add_inst(&mut self, inst_id: InstructionId) {
        self.instructions.push(inst_id)
    }
}
