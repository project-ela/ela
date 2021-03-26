use std::fmt;

use id_arena::Id;

use super::{BlockId, Value};

pub type InstructionId = Id<Instruction>;

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Value, Value),
    Equal(Value, Value),
    Ret(Value),
    Br(BlockId),
    CondBr(Value, BlockId, BlockId),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Instruction::*;

        match self {
            Add(lhs, rhs) => write!(f, "add {}, {}", lhs, rhs),
            Equal(lhs, rhs) => write!(f, "eq {}, {}", lhs, rhs),
            Ret(val) => write!(f, "ret {}", val),
            Br(dst) => write!(f, "br b{}", dst.index()),
            CondBr(cond, con, alt) => {
                write!(f, "br {} -> b{} b{} ", cond, con.index(), alt.index())
            }
        }
    }
}
