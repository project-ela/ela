use id_arena::{Arena, Id};

use super::{Block, BlockId, Instruction, InstructionId, InstructionKind, Type};

pub type FunctionId = Id<Function>;

#[derive(Debug)]
pub struct Function {
    pub name: String,

    pub param_typ: Vec<Type>,

    pub ret_typ: Type,

    pub blocks: Arena<Block>,

    pub block_order: Vec<BlockId>,

    pub instructions: Arena<Instruction>,
}

impl Function {
    pub fn new<S: Into<String>>(name: S, ret_typ: Type, param_typ: Vec<Type>) -> Self {
        let mut instructions = Arena::new();
        for (i, _) in param_typ.iter().enumerate() {
            instructions.alloc_with_id(|id| Instruction::new(id, InstructionKind::Param(i)));
        }

        Self {
            name: name.into(),
            param_typ,
            ret_typ,
            blocks: Arena::new(),
            block_order: Vec::new(),
            instructions,
        }
    }

    pub fn new_block(&mut self) -> BlockId {
        let block_id = self.blocks.alloc(Block::new());
        self.block_order.push(block_id);
        block_id
    }

    pub fn block(&self, id: BlockId) -> Option<&Block> {
        self.blocks.get(id)
    }

    pub fn block_mut(&mut self, id: BlockId) -> Option<&mut Block> {
        self.blocks.get_mut(id)
    }

    pub fn add_inst(&mut self, inst_kind: InstructionKind) -> InstructionId {
        let inst_id = self.instructions.alloc_with_id(|id| {
            let inst = Instruction::new(id, inst_kind);
            inst
        });

        let inst = self.inst(inst_id).unwrap();
        self.update_users_inst(inst.uses(), inst_id);

        inst_id
    }

    fn update_users_inst(&mut self, uses: Vec<InstructionId>, user_id: InstructionId) {
        for inst_id in uses {
            self.instructions
                .get_mut(inst_id)
                .unwrap()
                .add_user(user_id);
        }
    }

    pub fn inst(&self, inst_id: InstructionId) -> Option<&Instruction> {
        self.instructions.get(inst_id)
    }

    pub fn inst_mut(&mut self, inst_id: InstructionId) -> Option<&mut Instruction> {
        self.instructions.get_mut(inst_id)
    }
}
