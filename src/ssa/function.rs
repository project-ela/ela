use id_arena::{Arena, Id};

use super::{Block, BlockId, Instruction, InstructionId, Terminator, TerminatorId, Type, Types};

pub type FunctionId = Id<Function>;

#[derive(Debug)]
pub struct Function {
    pub name: String,

    pub param_typ: Vec<Type>,

    pub ret_typ: Type,

    pub blocks: Arena<Block>,

    pub instructions: Arena<Instruction>,

    pub terminators: Arena<Terminator>,

    pub types: Types,
}

impl Function {
    pub fn new(name: &str, ret_typ: Type, param_typ: Vec<Type>) -> Self {
        let mut instructions = Arena::new();
        for (i, _) in param_typ.iter().enumerate() {
            instructions.alloc(Instruction::Param(i));
        }

        Self {
            name: name.into(),
            param_typ,
            ret_typ,
            blocks: Arena::new(),
            instructions,
            terminators: Arena::new(),
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

    pub fn add_term(&mut self, term: Terminator) -> TerminatorId {
        self.terminators.alloc(term)
    }

    pub fn term(&self, term_id: TerminatorId) -> Option<&Terminator> {
        self.terminators.get(term_id)
    }
}
