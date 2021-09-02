use x86asm::instruction::{mnemonic::Mnemonic, operand::register::Register};

pub struct Program {
    pub insts: Vec<InstructionNode>,
}

#[derive(Debug)]
pub enum InstructionNode {
    PseudoOp(PseudoOp, Vec<PseudoOpArg>),
    Label(String),
    NullaryOp(Mnemonic),
    UnaryOp(Mnemonic, OperandNode),
    BinaryOp(Mnemonic, OperandNode, OperandNode),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum PseudoOp {
    Global,
    IntelSyntax,
    Data,
    Text,
    Zero,
    Ascii,
    Tse,
}

#[derive(Debug)]
pub enum PseudoOpArg {
    String(String),
    Integer(i32),
}

impl PseudoOpArg {
    pub fn as_string(&self) -> &String {
        match self {
            PseudoOpArg::String(s) => s,
            _ => panic!(),
        }
    }

    pub fn as_integer(&self) -> &i32 {
        match self {
            PseudoOpArg::Integer(i) => i,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OperandNode {
    Immidiate(i32),
    Register(Register),
    Label(String),
    Memory(MemoryNode),
}

#[derive(Debug, Clone)]
pub struct MemoryNode {
    pub base: Register,
    pub disp: Option<DispNode>,
}

#[derive(Debug, Clone)]
pub enum DispNode {
    Immediate(i32),
    Label(String),
}
