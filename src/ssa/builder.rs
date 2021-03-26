use super::{Block, BlockId, Function, Instruction, Value};

#[derive(Debug)]
pub struct FunctionBuilder<'a> {
    pub function: &'a mut Function,

    pub current_block: Option<BlockId>,
}

impl<'a> FunctionBuilder<'a> {
    pub fn new(function: &'a mut Function) -> Self {
        Self {
            function,
            current_block: None,
        }
    }

    pub fn add_block(&mut self) -> BlockId {
        self.function.add_block()
    }

    pub fn set_block(&mut self, block_id: BlockId) {
        self.current_block = Some(block_id);
    }

    pub fn current_block(&mut self) -> &mut Block {
        let block_id = self.current_block.unwrap();
        self.function.block_mut(block_id).unwrap()
    }

    pub fn add(&mut self, lhs: Value, rhs: Value) -> Value {
        let instruction = self.function.add_instruction(Instruction::Add(lhs, rhs));
        self.current_block().add_instruction(instruction);
        Value::new_inst(instruction, lhs.typ())
    }

    pub fn ret(&mut self, val: Value) {
        let instruction = self.function.add_instruction(Instruction::Ret(val));
        self.current_block().add_instruction(instruction);
    }
}
