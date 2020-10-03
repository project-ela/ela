use crate::common::operator::{BinaryOperator, UnaryOperator};

#[derive(Debug, Default)]
pub struct TacProgram {
    pub functions: Vec<TacFunction>,
}

#[derive(Debug)]
pub struct TacFunction {
    pub name: String,
    pub params: Vec<u32>,
    pub body: Vec<Tac>,
    pub stack_offset: u32,
}

#[derive(Debug)]
pub struct Parameter {
    pub offset: u32,
}

impl TacFunction {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            body: Vec::new(),
            stack_offset: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Tac {
    UnOp {
        op: UnaryOperator,
        src: Operand,
    },
    BinOp {
        op: BinaryOperator,
        dst: Operand,
        lhs: Operand,
        rhs: Operand,
    },
    Call {
        dst: Option<Operand>,
        name: String,
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
        src: Option<Operand>,
    },
}

#[derive(Debug, Copy, Clone)]
pub enum Operand {
    Reg(RegisterInfo),
    Const(i32),
    Variable(u32),
    Parameter(u32),
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

#[derive(Debug, Copy, Clone)]
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

impl TacProgram {
    pub fn dump(&self) -> String {
        let mut s = String::new();
        for function in &self.functions {
            s.push_str(function.dump().as_str());
        }
        s
    }
}

impl TacFunction {
    pub fn dump(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("func {}({}) {{\n", self.name, self.dump_params()).as_str());
        for tac in &self.body {
            s.push_str(tac.dump().as_str());
            s.push('\n');
        }
        s.push_str("}\n");
        s
    }

    fn dump_params(&self) -> String {
        self.params
            .iter()
            .map(|offset| format!("param({})", offset))
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl Tac {
    pub fn dump(&self) -> String {
        match self {
            Tac::UnOp { op, src } => format!("  {} = {:?} {}", src.dump(), op, src.dump()),
            Tac::BinOp { op, dst, lhs, rhs } => {
                format!("  {} = {} {:?} {}", dst.dump(), lhs.dump(), op, rhs.dump())
            }
            Tac::Call { dst, name } => match dst {
                Some(dst) => format!("  {} = call {}", dst.dump(), name),
                None => format!("  call {}", name),
            },
            Tac::Move { dst, src } => format!("  {} = {}", dst.dump(), src.dump()),
            Tac::Jump { label_index } => format!("  jmp label {}", label_index),
            Tac::JumpIfNot { label_index, cond } => {
                format!("  jmpifnot {}, label {}", cond.dump(), label_index)
            }
            Tac::Label { index } => format!("{}:", index),
            Tac::Ret { src } => match src {
                Some(src) => format!("  ret {}", src.dump()),
                None => "  ret".to_string(),
            },
        }
    }
}

impl Operand {
    pub fn dump(&self) -> String {
        match self {
            Operand::Reg(info) => format!(
                "%{}({})",
                info.virtual_index,
                info.physical_index.map_or("none", |reg| reg.dump())
            ),
            Operand::Const(value) => format!("{}", value),
            Operand::Variable(offset) => format!("var({})", offset),
            Operand::Parameter(offst) => format!("param({})", offst),
        }
    }
}

impl Register {
    pub fn dump(&self) -> &'static str {
        match self {
            Register::Eax => "eax",
            Register::Ecx => "ecx",
            Register::Edx => "edx",
            Register::Ebx => "ebx",
        }
    }
}
