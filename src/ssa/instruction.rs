use std::fmt;

use id_arena::Id;

use super::Value;

pub type InstructionId = Id<Instruction>;

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Value, Value),
    Ret(Value),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Instruction::*;

        match self {
            Add(lhs, rhs) => write!(f, "add {} {}", lhs, rhs),
            Ret(val) => write!(f, "ret {}", val),
        }
    }
}
