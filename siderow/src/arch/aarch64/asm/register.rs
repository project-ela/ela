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

    pub fn is_virtual(&self) -> bool {
        matches!(self.kind, RegisterKind::Virtual(_))
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
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
}

pub const REGS: [MachineRegisterKind; 16] = [
    MachineRegisterKind::X0,
    MachineRegisterKind::X1,
    MachineRegisterKind::X2,
    MachineRegisterKind::X3,
    MachineRegisterKind::X4,
    MachineRegisterKind::X5,
    MachineRegisterKind::X6,
    MachineRegisterKind::X7,
    MachineRegisterKind::X8,
    MachineRegisterKind::X9,
    MachineRegisterKind::X10,
    MachineRegisterKind::X11,
    MachineRegisterKind::X12,
    MachineRegisterKind::X13,
    MachineRegisterKind::X14,
    MachineRegisterKind::X15,
];

impl Printer for MachineRegisterKind {
    fn print(&self, buf: &mut String) -> super::Result {
        use self::MachineRegisterKind::*;

        let s = match self {
            X0 => "x0",
            X1 => "x1",
            X2 => "x2",
            X3 => "x3",
            X4 => "x4",
            X5 => "x5",
            X6 => "x6",
            X7 => "x7",
            X8 => "x8",
            X9 => "x9",
            X10 => "x10",
            X11 => "x11",
            X12 => "x12",
            X13 => "x13",
            X14 => "x14",
            X15 => "x15",
        };
        write!(buf, "{}", s)
    }
}

impl From<MachineRegisterKind> for Register {
    fn from(reg: MachineRegisterKind) -> Self {
        Register::new_physical(reg)
    }
}
