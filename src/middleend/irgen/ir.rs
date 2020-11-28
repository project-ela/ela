use crate::common::operator::{BinaryOperator, UnaryOperator};

#[derive(Debug, Default)]
pub struct IRProgram {
    pub functions: Vec<IRFunction>,
}

#[derive(Debug)]
pub struct IRFunction {
    pub name: String,
    pub params: Vec<u32>,
    pub blocks: Vec<IRBlock>,
    pub stack_offset: u32,
}

#[derive(Debug)]
pub struct Parameter {
    pub offset: u32,
}

impl IRFunction {
    pub fn new(name: String) -> Self {
        Self {
            name,
            params: Vec::new(),
            blocks: Vec::new(),
            stack_offset: 0,
        }
    }
}

#[derive(Debug)]
pub struct IRBlock {
    pub name: String,
    pub irs: Vec<IR>,
}

impl IRBlock {
    pub fn new(name: String) -> Self {
        Self {
            name,
            irs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum IR {
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
        args: Vec<Operand>,
    },
    Move {
        dst: Operand,
        src: Operand,
    },
    Jump {
        label: String,
    },
    JumpIfNot {
        label: String,
        cond: Operand,
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
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rdi,
    Rsi,
    Rbp,
    Rsp,

    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl IRProgram {
    pub fn dump(&self) -> String {
        let mut s = String::new();
        for function in &self.functions {
            s.push_str(function.dump().as_str());
        }
        s
    }
}

impl IRFunction {
    pub fn dump(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("func {}({}) {{\n", self.name, self.dump_params()).as_str());
        for block in &self.blocks {
            s.push_str(block.dump().as_str());
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

impl IRBlock {
    pub fn dump(&self) -> String {
        let mut s = String::new();
        s.push_str(format!("{}:\n", self.name).as_str());
        for ir in &self.irs {
            s.push_str(ir.dump().as_str());
            s.push('\n');
        }
        s
    }
}

impl IR {
    pub fn dump(&self) -> String {
        match self {
            IR::UnOp { op, src } => format!("  {} = {:?} {}", src.dump(), op, src.dump()),
            IR::BinOp { op, dst, lhs, rhs } => {
                format!("  {} = {} {:?} {}", dst.dump(), lhs.dump(), op, rhs.dump())
            }
            IR::Call { dst, name, args } => {
                let args = args
                    .iter()
                    .map(|arg| arg.dump())
                    .collect::<Vec<String>>()
                    .join(", ");
                match dst {
                    Some(dst) => format!("  {} = call {}({})", dst.dump(), name, args),
                    None => format!("  call {}({})", name, args),
                }
            }
            IR::Move { dst, src } => format!("  {} = {}", dst.dump(), src.dump()),
            IR::Jump { label } => format!("  jmp label {}", label),
            IR::JumpIfNot { label, cond } => format!("  jmpifnot {}, label {}", cond.dump(), label),
            IR::Ret { src } => match src {
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
            Register::Rax => "rax",
            Register::Rbx => "rbx",
            Register::Rcx => "rcx",
            Register::Rdx => "rdx",
            Register::Rdi => "rdi",
            Register::Rsi => "rsi",
            Register::Rbp => "rbp",
            Register::Rsp => "rsp",

            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
        }
    }
}
