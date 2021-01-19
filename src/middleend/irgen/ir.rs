use crate::common::{
    operator::{BinaryOperator, UnaryOperator},
    types::Type,
};

#[derive(Debug, Default)]
pub struct IRProgram {
    pub global_defs: Vec<IRGlobalDef>,
    pub functions: Vec<IRFunction>,
}

#[derive(Debug)]
pub struct IRGlobalDef {
    pub name: String,
    pub typ: Type,
    pub init_value: Option<String>,
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
    Addr {
        dst: Operand,
        src: i32,
    },
    AddrLabel {
        dst: Operand,
        src: String,
    },
    Load {
        dst: Operand,
        src: Operand,
        size: RegSize,
    },
    Store {
        dst: Operand,
        src: Operand,
        size: RegSize,
    },
    StoreArg {
        dst: i32,
        src: usize, // index of argument
        size: RegSize,
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
}

impl Operand {
    pub fn is_reg(&self) -> bool {
        matches!(self, Operand::Reg(_))
    }

    pub fn is_const(&self) -> bool {
        matches!(self, Operand::Const(_))
    }

    pub fn is_same(&self, other: &Operand) -> bool {
        match (self, other) {
            (Operand::Reg(info1), Operand::Reg(info2)) => info1.is_same_reg(info2),
            (Operand::Const(value1), Operand::Const(value2)) => value1 == value2,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RegisterInfo {
    pub virtual_index: u32,
    pub physical_index: Option<Register>,
}

impl RegisterInfo {
    pub fn is_same_reg(&self, other: &RegisterInfo) -> bool {
        self.physical_index == other.physical_index
    }
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

#[derive(Debug, Copy, Clone)]
pub enum RegSize {
    Byte,
    Word,
    DWord,
    QWord,
}

impl From<&Type> for RegSize {
    fn from(typ: &Type) -> Self {
        let size = match typ {
            Type::Array { elm_type, .. } => elm_type.size(),
            x => x.size(),
        };

        match size {
            1 => RegSize::Byte,
            2 => RegSize::Word,
            4 => RegSize::DWord,
            8 => RegSize::QWord,
            _ => panic!(),
        }
    }
}

impl From<Type> for RegSize {
    fn from(typ: Type) -> Self {
        Self::from(&typ)
    }
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
            IR::Addr { dst, src } => format!("  {} = {}", dst.dump(), src),
            IR::AddrLabel { dst, src } => format!("  {} = {}", dst.dump(), src),
            IR::Load { dst, src, size: _ } => format!("  {} = [{}]", dst.dump(), src.dump()),
            IR::Store { dst, src, size: _ } => format!("  [{}] = {}", dst.dump(), src.dump()),
            IR::StoreArg { dst, src, size: _ } => format!("  {} = param({})", dst, src),
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
