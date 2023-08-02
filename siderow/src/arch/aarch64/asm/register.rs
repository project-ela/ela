use std::fmt::Write;

use super::Printer;

#[derive(Debug)]
pub enum MachineRegisterKind {
    X0,
}

impl Printer for MachineRegisterKind {
    fn print(&self, buf: &mut String) -> super::Result {
        use self::MachineRegisterKind::*;

        let s = match self {
            X0 => "x0",
        };
        write!(buf, "{}", s)
    }
}
