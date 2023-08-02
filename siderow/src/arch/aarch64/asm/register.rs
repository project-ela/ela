use std::fmt::Write;

use super::Printer;

#[derive(Debug, Clone)]
pub struct Register {
    pub kind: RegisterKind,
    // TODO: size
}

impl Register {
    pub fn new_virtual(id: usize) -> Self {
        Self {
            kind: RegisterKind::Virtual(id),
        }
    }

    pub fn new_physical(kind: MachineRegisterKind) -> Self {
        Self {
            kind: RegisterKind::Physical(kind),
        }
    }
}

impl Printer for Register {
    fn print(&self, buf: &mut String) -> super::Result {
        self.kind.print(buf)
    }
}

#[derive(Debug, Clone)]
pub enum RegisterKind {
    Virtual(usize),
    Physical(MachineRegisterKind),
}

impl Printer for RegisterKind {
    fn print(&self, buf: &mut String) -> super::Result {
        use self::RegisterKind::*;

        match self {
            Virtual(id) => write!(buf, "%{}", id),
            Physical(kind) => kind.print(buf),
        }
    }
}

#[derive(Debug, Clone)]
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
