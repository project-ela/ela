use id_arena::Id;

use super::{BlockId, FunctionId, Type, Value};

pub type InstructionId = Id<Instruction>;

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Value, Value),
    Equal(Value, Value),

    Call(FunctionId),

    Alloc(Type),
    Load(Value),
    Store(Value, Value),

    Ret(Value),
    Br(BlockId),
    CondBr(Value, BlockId, BlockId),
}
