use super::{Constant, Function, GlobalId, InstructionId, Module, Type};

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Constant(Constant),
    Instruction(InstructionValue),
    Parameter(ParameterValue),
    Global(GlobalValue),
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

#[derive(Debug, Clone, Copy)]
pub struct GlobalValue {
    pub global_id: GlobalId,
    pub typ: Type,
}

impl Value {
    pub fn new_inst(inst_id: InstructionId, typ: Type) -> Self {
        Self::Instruction(InstructionValue { inst_id, typ })
    }

    pub fn new_param(function: &Function, index: usize) -> Self {
        let typ = function.param_typ.get(index).unwrap().clone();
        Self::Parameter(ParameterValue { index, typ })
    }

    pub fn new_global(module: &Module, global_id: GlobalId) -> Self {
        let typ = module.global(global_id).unwrap().typ;
        Self::Global(GlobalValue { global_id, typ })
    }

    pub fn typ(&self) -> Type {
        use self::Value::*;

        match self {
            Constant(r#const) => r#const.typ(),
            Instruction(InstructionValue { typ, .. }) => *typ,
            Parameter(ParameterValue { typ, .. }) => *typ,
            Global(GlobalValue { typ, .. }) => *typ,
        }
    }
}
