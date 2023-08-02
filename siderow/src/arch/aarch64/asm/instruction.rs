use std::fmt::Write;

use super::Printer;

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
    fn print(&self, buf: &mut String) {
        self.mnemonic.print(buf);
        write!(buf, " ");
        for (i, operand) in self.operands.iter().enumerate() {
            if i != 0 {
                write!(buf, ", ");
            }
            operand.print(buf);
        }
        writeln!(buf, "");
    }
}

#[derive(Debug)]
pub enum Mnemonic {
    B,
    Ret,
}

impl Printer for Mnemonic {
    fn print(&self, buf: &mut String) {
        use self::Mnemonic::*;

        let s = match self {
            B => "b",
            Ret => "ret",
        };
        write!(buf, "{}", s);
    }
}

#[derive(Debug)]
pub enum Operand {
    Label(String),
}

impl Printer for Operand {
    fn print(&self, buf: &mut String) {
        use self::Operand::*;

        match self {
            Label(name) => write!(buf, "{}", name),
        };
    }
}
