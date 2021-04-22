use super::{Immediate, Indirect, RegSize, Register};

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
    Lea,
    Mov,
    Movzx,
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

impl Operand {
    pub fn virt_regs(&self) -> Option<Vec<&Register>> {
        match self {
            Self::Register(reg @ Register::Virtual(_)) => Some(vec![reg]),
            Self::Indirect(Indirect { base, index, .. }) => {
                let mut regs = vec![];
                if let Register::Virtual(_) = base {
                    regs.push(base);
                }
                if let Some(reg @ Register::Virtual(_)) = index {
                    regs.push(reg);
                }
                Some(regs)
            }
            _ => None,
        }
    }

    pub fn virt_regs_mut(&mut self) -> Option<Vec<&mut Register>> {
        match self {
            Self::Register(ref mut reg @ Register::Virtual(_)) => Some(vec![reg]),
            Self::Indirect(Indirect { base, index, .. }) => {
                let mut regs = vec![];
                if let Register::Virtual(_) = base {
                    regs.push(base);
                }
                if let Some(reg @ Register::Virtual(_)) = index {
                    regs.push(reg);
                }
                Some(regs)
            }
            _ => None,
        }
    }

    pub fn size(&self) -> RegSize {
        match self {
            Self::Register(_) => RegSize::QWord,
            Self::Immediate(_) => RegSize::QWord,
            Self::Indirect(indirect) => indirect.size,
            x => panic!("{:?}", x),
        }
    }
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
            Lea => "lea",
            Mov => "mov",
            Movzx => "movzx",
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
