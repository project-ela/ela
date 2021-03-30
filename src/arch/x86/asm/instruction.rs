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
    Cqo,
    Idiv,
    Imul,
    Je,
    Jmp,
    Mov,
    Or,
    Pop,
    Push,
    Ret,
    Sete,
    Setg,
    Setge,
    Setl,
    Setle,
    Setne,
    Shl,
    Shr,
    Sub,
    Xor,
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
            Cqo => "cqo",
            Idiv => "idiv",
            Imul => "imul",
            Je => "je",
            Jmp => "jmp",
            Mov => "mov",
            Or => "or",
            Pop => "pop",
            Push => "push",
            Ret => "ret",
            Sete => "sete",
            Setg => "setg",
            Setge => "setge",
            Setl => "setl",
            Setle => "setle",
            Setne => "setne",
            Shl => "shl",
            Shr => "shr",
            Sub => "sub",
            Xor => "xor",
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
