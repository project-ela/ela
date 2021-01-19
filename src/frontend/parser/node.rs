use x86asm::instruction::{mnemonic::Mnemonic, operand::register::Register};

pub struct Program {
    pub insts: Vec<InstructionNode>,
}

#[derive(Debug)]
pub enum InstructionNode {
    PseudoOp(PseudoOp, Option<String>),
    Label { name: String },
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
}

#[derive(Debug, Clone)]
pub enum OperandNode {
    Immidiate { value: u32 },
    Register { reg: Register },
    Label { name: String },
    Memory(MemoryNode),
}

#[derive(Debug, Clone)]
pub struct MemoryNode {
    pub base: Register,
    pub disp: Option<i32>,
}
