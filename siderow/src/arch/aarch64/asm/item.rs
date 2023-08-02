use std::fmt::Write;

use crate::arch::aarch64::asm::Result;

use super::{Instruction, Printer};

#[derive(Debug)]
pub enum AssemblyItem {
    Instruction(Instruction),
    Label(String),
}

impl Printer for AssemblyItem {
    fn print(&self, buf: &mut String) -> Result {
        use self::AssemblyItem::*;

        match self {
            Instruction(inst) => inst.print(buf),
            Label(name) => writeln!(buf, "{}:", name),
        }
    }
}
