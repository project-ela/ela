use id_arena::Id;

use super::InstructionId;

pub type BlockId = Id<Block>;

#[derive(Debug)]
pub struct Block {
    pub instructions: Vec<InstructionId>,

    pub terminator: Option<InstructionId>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            terminator: None,
        }
    }

    pub fn add_inst(&mut self, inst_id: InstructionId) {
        self.instructions.push(inst_id)
    }

    pub fn set_term(&mut self, inst_id: InstructionId) {
        self.terminator = Some(inst_id);
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }

    pub fn is_terminated(&self) -> bool {
        self.terminator.is_some()
    }
}
