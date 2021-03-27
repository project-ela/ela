use id_arena::Arena;

use super::{Block, BlockId, Instruction, InstructionId, Types};

#[derive(Debug)]
pub struct Function {
    pub name: String,

    pub blocks: Arena<Block>,

    pub instructions: Arena<Instruction>,

    pub types: Types,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            blocks: Arena::new(),
            instructions: Arena::new(),
            types: Types::new(),
        }
    }

    pub fn add_block(&mut self) -> BlockId {
        self.blocks.alloc(Block::new())
    }

    pub fn block_mut(&mut self, id: BlockId) -> Option<&mut Block> {
        self.blocks.get_mut(id)
    }

    pub fn add_inst(&mut self, inst: Instruction) -> InstructionId {
        self.instructions.alloc(inst)
    }

    pub fn inst(&self, inst_id: InstructionId) -> Option<&Instruction> {
        self.instructions.get(inst_id)
    }
}
