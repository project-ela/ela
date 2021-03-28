use id_arena::Id;

use super::{InstructionId, TerminatorId};

pub type BlockId = Id<Block>;

#[derive(Debug)]
pub struct Block {
    pub instructions: Vec<InstructionId>,

    pub terminator: Option<TerminatorId>,
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

    pub fn set_term(&mut self, term_id: TerminatorId) {
        self.terminator = Some(term_id);
    }
}
