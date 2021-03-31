use id_arena::{Arena, Id};

use super::{
    Block, BlockId, Instruction, InstructionId, InstructionKind, Terminator, TerminatorId, Type,
    Types,
};

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
            instructions.alloc(Instruction::new(InstructionKind::Param(i)));
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

    pub fn block(&self, id: BlockId) -> Option<&Block> {
        self.blocks.get(id)
    }

    pub fn block_mut(&mut self, id: BlockId) -> Option<&mut Block> {
        self.blocks.get_mut(id)
    }

    pub fn add_inst(&mut self, inst_kind: InstructionKind) -> InstructionId {
        let inst = Instruction::new(inst_kind);
        self.update_users_inst(&inst, self.instructions.next_id());
        self.instructions.alloc(inst)
    }

    fn update_users_inst(&mut self, user: &Instruction, user_id: InstructionId) {
        for inst_id in user.uses() {
            self.instructions
                .get_mut(inst_id)
                .unwrap()
                .add_user_inst(user_id);
        }
    }

    pub fn inst(&self, inst_id: InstructionId) -> Option<&Instruction> {
        self.instructions.get(inst_id)
    }

    pub fn inst_mut(&mut self, inst_id: InstructionId) -> Option<&mut Instruction> {
        self.instructions.get_mut(inst_id)
    }

    pub fn add_term(&mut self, term: Terminator) -> TerminatorId {
        self.update_users_term(&term, self.terminators.next_id());
        self.terminators.alloc(term)
    }

    fn update_users_term(&mut self, user: &Terminator, user_id: TerminatorId) {
        for inst_id in user.uses() {
            self.instructions
                .get_mut(inst_id)
                .unwrap()
                .add_user_term(user_id);
        }
    }

    pub fn term(&self, term_id: TerminatorId) -> Option<&Terminator> {
        self.terminators.get(term_id)
    }

    pub fn term_mut(&mut self, term_id: TerminatorId) -> Option<&mut Terminator> {
        self.terminators.get_mut(term_id)
    }
}
