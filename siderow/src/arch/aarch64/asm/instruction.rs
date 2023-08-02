use std::fmt::Write;

use crate::arch::aarch64::asm::Result;

use super::{Immediate, MachineRegisterKind, Printer};

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

impl Printer for Instruction {
    fn print(&self, buf: &mut String) -> Result {
        self.mnemonic.print(buf)?;
        write!(buf, " ")?;
        for (i, operand) in self.operands.iter().enumerate() {
            if i != 0 {
                write!(buf, ", ")?;
            }
            operand.print(buf)?;
        }
        writeln!(buf, "")
    }
}

#[derive(Debug)]
pub enum Mnemonic {
    B,
    Mov,
    Ret,
}

impl Printer for Mnemonic {
    fn print(&self, buf: &mut String) -> Result {
        use self::Mnemonic::*;

        let s = match self {
            B => "b",
            Mov => "mov",
            Ret => "ret",
        };
        write!(buf, "{}", s)
    }
}

#[derive(Debug)]
pub enum Operand {
    Register(MachineRegisterKind),
    Label(String),
    Immediate(Immediate),
}

impl Printer for Operand {
    fn print(&self, buf: &mut String) -> Result {
        use self::Operand::*;

        match self {
            Register(reg) => reg.print(buf),
            Label(name) => write!(buf, "{}", name),
            Immediate(imm) => imm.print(buf),
        }
    }
}
