use std::collections::HashSet;

use id_arena::Id;

use super::{FunctionId, TerminatorId, Type, Value};

pub type InstructionId = Id<Instruction>;

#[derive(Debug)]
pub struct Instruction {
    pub kind: InstructionKind,

    pub users_inst: HashSet<InstructionId>,
    pub users_term: HashSet<TerminatorId>,
}

impl Instruction {
    pub fn new(kind: InstructionKind) -> Self {
        Self {
            kind,
            users_inst: HashSet::new(),
            users_term: HashSet::new(),
        }
    }

    pub fn uses(&self) -> Vec<InstructionId> {
        let mut uses = HashSet::new();
        for value in self.kind.values() {
            if let Value::Instruction(inst_val) = value {
                uses.insert(inst_val.inst_id);
            }
        }
        uses.into_iter().collect()
    }

    pub fn add_user_inst(&mut self, user: InstructionId) {
        self.users_inst.insert(user);
    }

    pub fn add_user_term(&mut self, user: TerminatorId) {
        self.users_term.insert(user);
    }
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
}

impl InstructionKind {
    pub fn values(&self) -> Vec<&Value> {
        use self::InstructionKind::*;

        match self {
            BinOp(_, lhs, rhs) => vec![lhs, rhs],
            Cmp(_, lhs, rhs) => vec![lhs, rhs],

            Call(_, args) => args.iter().collect(),
            Param(_) => vec![],

            Alloc(_) => vec![],
            Load(src) => vec![src],
            Store(dst, src) => vec![dst, src],
        }
    }
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
