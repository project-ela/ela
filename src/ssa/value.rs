use super::{Function, Immediate, InstructionId, Type};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Immediate(Immediate),
    Instruction(InstructionValue),
    Parameter(ParameterValue),
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionValue {
    pub inst_id: InstructionId,
    pub typ: Type,
}

#[derive(Debug, Clone, Copy)]
pub struct ParameterValue {
    pub index: usize,
    pub typ: Type,
}

impl Value {
    pub fn new_inst(inst_id: InstructionId, typ: Type) -> Self {
        Self::Instruction(InstructionValue { inst_id, typ })
    }

    pub fn new_param(function: &Function, index: usize) -> Self {
        let param_typ = function.param_typ.get(index).unwrap().clone();
        Self::Parameter(ParameterValue {
            index,
            typ: param_typ,
        })
    }

    pub fn typ(&self) -> Type {
        use self::Value::*;

        match self {
            Immediate(imm) => imm.typ(),
            Instruction(InstructionValue { typ, .. }) => *typ,
            Parameter(ParameterValue { typ, .. }) => *typ,
        }
    }
}
