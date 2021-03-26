use std::fmt;

use super::{Immediate, InstructionId, Type};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Immediate(Immediate),
    Instruction(InstructionValue),
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionValue {
    pub inst_id: InstructionId,
    pub typ: Type,
}

impl Value {
    pub fn new_inst(inst_id: InstructionId, typ: Type) -> Self {
        Self::Instruction(InstructionValue { inst_id, typ })
    }

    pub fn typ(&self) -> Type {
        use self::Value::*;

        match self {
            Immediate(imm) => imm.typ(),
            Instruction(InstructionValue { typ, .. }) => *typ,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Value::*;

        match self {
            Immediate(imm) => write!(f, "{}", imm),
            Instruction(inst) => write!(f, "{} %{}", inst.typ, inst.inst_id.index()),
        }
    }
}
