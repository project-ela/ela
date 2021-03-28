use id_arena::Id;

use super::{BlockId, FunctionId, Type, Value};

pub type InstructionId = Id<Instruction>;

#[derive(Debug)]
pub enum Instruction {
    BinOp(BinaryOperator, Value, Value),
    Cmp(ComparisonOperator, Value, Value),

    Call(FunctionId, Vec<Value>),
    Arg(usize),

    Alloc(Type),
    Load(Value),
    Store(Value, Value),
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

pub type TerminatorId = Id<Terminator>;

#[derive(Debug)]
pub enum Terminator {
    Ret(Option<Value>),
    Br(BlockId),
    CondBr(Value, BlockId, BlockId),
}
