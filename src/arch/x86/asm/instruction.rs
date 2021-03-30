use super::{Immediate, Indirect, Register};

#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operands: Vec<Operand>,
}

impl Instruction {
    pub fn new(mnemonic: Mnemonic, operands: Vec<Operand>) -> Self {
        Self { mnemonic, operands }
    }
}

#[derive(Debug)]
pub enum Mnemonic {
    Add,
    And,
    Call,
    Cmp,
    Je,
    Jmp,
    Mov,
    Pop,
    Push,
    Ret,
    Sete,
    Sub,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Register(Register),
    Label(String),
    Immediate(Immediate),
    Indirect(Indirect),
}

impl Instruction {
    pub fn stringify(&self) -> String {
        let operands_str = self
            .operands
            .iter()
            .map(|operand| operand.stringify())
            .collect::<Vec<String>>()
            .join(", ");

        format!("  {} {}", self.mnemonic.stringify(), operands_str)
    }
}

impl Mnemonic {
    pub fn stringify(&self) -> String {
        use self::Mnemonic::*;

        match self {
            Add => "add",
            And => "and",
            Call => "call",
            Cmp => "cmp",
            Je => "je",
            Jmp => "jmp",
            Mov => "mov",
            Pop => "pop",
            Push => "push",
            Ret => "ret",
            Sete => "sete",
            Sub => "sub",
        }
        .into()
    }
}

impl Operand {
    pub fn stringify(&self) -> String {
        use self::Operand::*;

        match self {
            Register(reg) => reg.stringify(),
            Label(name) => name.clone(),
            Immediate(imm) => imm.stringify(),
            Indirect(addr) => addr.stringify(),
        }
    }
}
