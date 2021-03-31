use std::collections::HashSet;

use id_arena::Id;

use super::{BlockId, FunctionId, Type, Value};

pub type InstructionId = Id<Instruction>;

#[derive(Debug)]
pub struct Instruction {
    pub kind: InstructionKind,

    pub users: HashSet<InstructionId>,
}

#[derive(Debug)]
pub enum InstructionKind {
    BinOp(BinaryOperator, Value, Value),
    Cmp(ComparisonOperator, Value, Value),

    Call(FunctionId, Vec<Value>),
    Param(usize),

    Alloc(Type),
    Load(Value),
    Store(Value, Value),

    // terminators
    Ret(Option<Value>),
    Br(BlockId),
    CondBr(Value, BlockId, BlockId),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,

    Shl,
    Shr,

    And,
    Or,
    Xor,
}

#[derive(Debug)]
pub enum ComparisonOperator {
    Eq,
    Neq,

    Gt,
    Gte,
    Lt,
    Lte,
}

impl Instruction {
    pub fn new(kind: InstructionKind) -> Self {
        Self {
            kind,
            users: HashSet::new(),
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

    pub fn add_user(&mut self, user: InstructionId) {
        self.users.insert(user);
    }

    pub fn values(&self) -> Vec<&Value> {
        use self::InstructionKind::*;

        match &self.kind {
            BinOp(_, lhs, rhs) => vec![lhs, rhs],
            Cmp(_, lhs, rhs) => vec![lhs, rhs],

            Call(_, args) => args.iter().collect(),
            Param(_) => vec![],

            Alloc(_) => vec![],
            Load(src) => vec![src],
            Store(dst, src) => vec![dst, src],

            Ret(None) => vec![],
            Ret(Some(val)) => vec![val],
            Br(_) => vec![],
            CondBr(cond, _, _) => vec![cond],
        }
    }

    pub fn values_mut(&mut self) -> Vec<&mut Value> {
        use self::InstructionKind::*;

        match &mut self.kind {
            BinOp(_, lhs, rhs) => vec![lhs, rhs],
            Cmp(_, lhs, rhs) => vec![lhs, rhs],

            Call(_, args) => args.iter_mut().collect(),
            Param(_) => vec![],

            Alloc(_) => vec![],
            Load(src) => vec![src],
            Store(dst, src) => vec![dst, src],

            Ret(None) => vec![],
            Ret(Some(val)) => vec![val],
            Br(_) => vec![],
            CondBr(cond, _, _) => vec![cond],
        }
    }

    pub fn is_terminator(&self) -> bool {
        use self::InstructionKind::*;

        match self.kind {
            Ret(_) | Br(_) | CondBr(_, _, _) => true,
            _ => false,
        }
    }

    pub fn has_side_effects(&self) -> bool {
        use self::InstructionKind::*;

        match self.kind {
            Call(_, _) | Param(_) => true,
            _ => false,
        }
    }
}
