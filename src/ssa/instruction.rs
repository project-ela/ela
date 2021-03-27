use id_arena::Id;

use super::{BlockId, Type, Value};

pub type InstructionId = Id<Instruction>;

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Value, Value),
    Equal(Value, Value),

    Alloc(Type),
    Load(Value),
    Store(Value, Value),

    Ret(Value),
    Br(BlockId),
    CondBr(Value, BlockId, BlockId),
}
