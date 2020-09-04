use crate::common::operator::Operator;

#[derive(Debug)]
pub struct TacProgram {
    pub functions: Vec<TacFunction>,
}

impl TacProgram {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct TacFunction {
    pub name: String,
    pub body: Vec<Tac>,
    pub stack_offset: u32,
}

impl TacFunction {
    pub fn new(name: String) -> Self {
        Self {
            name,
            body: Vec::new(),
            stack_offset: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Tac {
    BinOp {
        op: Operator,
        dst: Operand,
        lhs: Operand,
        rhs: Operand,
    },
    Move {
        dst: Operand,
        src: Operand,
    },
    Jump {
        label_index: u32,
    },
    JumpIfNot {
        label_index: u32,
        cond: Operand,
    },
    Label {
        index: u32,
    },
    Ret {
        src: Operand,
    },
}

#[derive(Debug, Clone)]
pub enum Operand {
    Reg(RegisterInfo),
    Const(u32),
    Variable(u32),
}

impl Operand {
    pub fn is_reg(&self) -> bool {
        match self {
            Operand::Reg(_) => true,
            _ => false,
        }
    }

    pub fn is_const(&self) -> bool {
        match self {
            Operand::Const(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisterInfo {
    pub virtual_index: u32,
    pub physical_index: Option<Register>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Register {
    Eax,
    Ecx,
    Edx,
    Ebx,
}
