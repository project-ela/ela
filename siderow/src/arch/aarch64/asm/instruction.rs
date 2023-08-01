use std::fmt::Write;

use super::Printer;

#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
}

impl Instruction {
    pub fn new(mnemonic: Mnemonic) -> Self {
        Self { mnemonic }
    }
}

impl Printer for Instruction {
    fn print(&self, buf: &mut String) {
        self.mnemonic.print(buf);
        writeln!(buf, "");
    }
}

#[derive(Debug)]
pub enum Mnemonic {
    Ret,
}

impl Printer for Mnemonic {
    fn print(&self, buf: &mut String) {
        use self::Mnemonic::*;

        let s = match self {
            Ret => "ret",
        };
        write!(buf, "{}", s);
    }
}
