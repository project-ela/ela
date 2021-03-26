use std::fmt;

use super::InstructionId;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Immediate(u32),
    Instruction(InstructionId),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Value::*;

        match self {
            Immediate(imm) => write!(f, "{}", imm),
            Instruction(inst) => write!(f, "%{}", inst.index()),
        }
    }
}
