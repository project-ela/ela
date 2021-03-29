use crate::ssa::InstructionId;

#[derive(Debug)]
pub enum Register {
    Virtual(usize),
    Physical(MachineRegister),
}

impl From<InstructionId> for Register {
    fn from(inst_id: InstructionId) -> Self {
        Register::Virtual(inst_id.index())
    }
}

impl From<&InstructionId> for Register {
    fn from(inst_id: &InstructionId) -> Self {
        Register::Virtual(inst_id.index())
    }
}

impl From<MachineRegister> for Register {
    fn from(reg: MachineRegister) -> Self {
        Register::Physical(reg)
    }
}

#[derive(Debug)]
pub enum MachineRegister {
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
}

impl Register {
    pub fn stringify(&self) -> String {
        use self::Register::*;

        match self {
            Virtual(id) => format!("%{}", id),
            Physical(reg) => reg.stringify(),
        }
    }
}

impl MachineRegister {
    pub fn stringify(&self) -> String {
        use self::MachineRegister::*;

        match self {
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
        }
        .into()
    }
}
