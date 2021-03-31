use std::collections::HashSet;

use id_arena::Id;

use super::{BlockId, InstructionId, Value};

pub type TerminatorId = Id<Terminator>;

#[derive(Debug)]
pub enum Terminator {
    Ret(Option<Value>),
    Br(BlockId),
    CondBr(Value, BlockId, BlockId),
}

impl Terminator {
    pub fn values(&self) -> Vec<&Value> {
        use self::Terminator::*;

        match self {
            Ret(None) => vec![],
            Ret(Some(val)) => vec![val],
            Br(_) => vec![],
            CondBr(cond, _, _) => vec![cond],
        }
    }

    pub fn values_mut(&mut self) -> Vec<&mut Value> {
        use self::Terminator::*;

        match self {
            Ret(None) => vec![],
            Ret(Some(val)) => vec![val],
            Br(_) => vec![],
            CondBr(cond, _, _) => vec![cond],
        }
    }

    pub fn uses(&self) -> Vec<InstructionId> {
        let mut uses = HashSet::new();
        for value in self.values() {
            if let Value::Instruction(inst_val) = value {
                uses.insert(inst_val.inst_id);
            }
        }
        uses.into_iter().collect()
    }
}
