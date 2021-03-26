use id_arena::Arena;

use super::{Block, BlockId, Instruction, InstructionId};

#[derive(Debug)]
pub struct Function {
    pub name: String,

    pub blocks: Arena<Block>,

    pub instructions: Arena<Instruction>,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            blocks: Arena::new(),
            instructions: Arena::new(),
        }
    }

    pub fn add_block(&mut self) -> BlockId {
        self.blocks.alloc(Block::new())
    }

    pub fn block_mut(&mut self, id: BlockId) -> Option<&mut Block> {
        self.blocks.get_mut(id)
    }

    pub fn add_instruction(&mut self, instruction: Instruction) -> InstructionId {
        self.instructions.alloc(instruction)
    }
}
