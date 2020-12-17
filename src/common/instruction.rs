use x86asm::instruction::{mnemonic::Mnemonic, operand::register::Register};

#[derive(Debug)]
pub enum Instruction {
    PseudoOp { name: String, arg: String },
    Label { name: String },
    NullaryOp(Mnemonic),
    UnaryOp(Mnemonic, Operand),
    BinaryOp(Mnemonic, Operand, Operand),
}

#[derive(Debug, Clone)]
pub enum Operand {
    Immidiate { value: u32 },
    Register { reg: Register },
    Label { name: String },
    Address(Address),
}
#[derive(Debug, Clone)]
pub struct Address {
    pub base: Register,
    pub disp: Option<i32>,
}
