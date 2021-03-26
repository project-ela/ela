use core::fmt;

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

    pub fn instruction(&self, id: InstructionId) -> Option<&Instruction> {
        self.instructions.get(id)
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "func {}() {{", self.name)?;

        for (i, block) in &self.blocks {
            writeln!(f, "  b{}:", i.index())?;

            for instruction_id in &block.instructions {
                let instruction = self.instruction(*instruction_id).unwrap();
                writeln!(f, "    %{} = {}", instruction_id.index(), instruction)?;
            }

            writeln!(f, "")?;
        }

        write!(f, "}}")?;

        Ok(())
    }
}
