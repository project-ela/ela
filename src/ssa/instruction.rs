use id_arena::Id;

use super::Value;

pub type InstructionId = Id<Instruction>;

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Value, Value),
    Ret(Value),
}
