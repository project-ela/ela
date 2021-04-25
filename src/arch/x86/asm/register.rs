use crate::ssa::InstructionId;

#[derive(Debug, Clone)]
pub struct Register {
    pub kind: RegisterKind,

    pub size: RegisterSize,
}

impl Register {
    pub fn new(kind: RegisterKind, size: RegisterSize) -> Self {
        Self { kind, size }
    }

    pub fn new_qword(kind: RegisterKind) -> Self {
        Self {
            kind,
            size: RegisterSize::QWord,
        }
    }

    pub fn set_size(&mut self, size: RegisterSize) {
        self.size = size;
    }
}

#[derive(Debug, Clone)]
pub enum RegisterKind {
    Virtual(usize),
    Physical(MachineRegisterKind),
}

impl From<InstructionId> for Register {
    fn from(inst_id: InstructionId) -> Self {
        Register::new_qword(RegisterKind::Virtual(inst_id.index()))
    }
}

impl From<&InstructionId> for Register {
    fn from(inst_id: &InstructionId) -> Self {
        Register::new_qword(RegisterKind::Virtual(inst_id.index()))
    }
}

impl From<MachineRegisterKind> for Register {
    fn from(reg: MachineRegisterKind) -> Self {
        Register::new_qword(RegisterKind::Physical(reg))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MachineRegisterKind {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rdi,
    Rsi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

    Rip,

    // TODO
    Cl,
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterSize {
    QWord,
    DWord,
    Word,
    Byte,
}

impl RegisterSize {
    pub fn size(&self) -> usize {
        use self::RegisterSize::*;

        match self {
            QWord => 8,
            DWord => 4,
            Word => 2,
            Byte => 1,
        }
    }
}

pub const REGS: [MachineRegisterKind; 7] = [
    MachineRegisterKind::R10,
    MachineRegisterKind::R11,
    MachineRegisterKind::Rbx,
    MachineRegisterKind::R12,
    MachineRegisterKind::R13,
    MachineRegisterKind::R14,
    MachineRegisterKind::R15,
];

impl Register {
    pub fn stringify(&self) -> String {
        use self::RegisterKind::*;

        match &self.kind {
            Virtual(id) => format!("%{}", id),
            Physical(reg) => reg.stringify(self.size),
        }
    }
}

impl MachineRegisterKind {
    pub fn stringify(&self, size: RegisterSize) -> String {
        use self::MachineRegisterKind::*;

        match size {
            RegisterSize::QWord => match self {
                Rax => "rax",
                Rbx => "rbx",
                Rcx => "rcx",
                Rdx => "rdx",
                Rdi => "rdi",
                Rsi => "rsi",
                Rbp => "rbp",
                Rsp => "rsp",
                R8 => "r8",
                R9 => "r9",
                R10 => "r10",
                R11 => "r11",
                R12 => "r12",
                R13 => "r13",
                R14 => "r14",
                R15 => "r15",

                Rip => "rip",

                Cl => "cl",
            },
            RegisterSize::Byte => match self {
                Rax => "al",
                Rbx => "bl",
                Rcx => "cl",
                Rdx => "dl",
                Rdi => "dil",
                Rsi => "sil",
                Rbp => "bpl",
                Rsp => "spl",
                R8 => "r8b",
                R9 => "r9b",
                R10 => "r10b",
                R11 => "r11b",
                R12 => "r12b",
                R13 => "r13b",
                R14 => "r14b",
                R15 => "r15b",

                Rip => "rip",

                Cl => "cl",
            },
            x => unimplemented!("{:?}", x),
        }
        .into()
    }
}
