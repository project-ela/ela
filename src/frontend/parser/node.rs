use x86asm::instruction::{mnemonic::Mnemonic, operand::register::Register};

#[derive(Debug)]
pub enum InstructionNode {
    PseudoOp(PseudoOp, String),
    Label { name: String },
    NullaryOp(Mnemonic),
    UnaryOp(Mnemonic, OperandNode),
    BinaryOp(Mnemonic, OperandNode, OperandNode),
}
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum PseudoOp {
    Global,
    IntelSyntax,
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
