#[derive(Debug, Default)]
pub struct Cpu {
    // Accumulator
    eax: u32,
    // Counter
    ecx: u32,
    // Data
    edx: u32,
    // Base
    ebx: u32,
    // Stack Pointer
    esp: u32,
    // Stack Base Pointer
    ebp: u32,
    // Source Index
    esi: u32,
    // Destination Index
    edi: u32,

    flags: u32,

    // Instruction Pointer
    eip: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
    EIP,
}

pub enum Flags {
    CF = 0,
    ZF = 6,
    SF = 7,
    OF = 11,
}

impl From<u8> for Register {
    fn from(index: u8) -> Self {
        match index {
            0 => Self::EAX,
            1 => Self::ECX,
            2 => Self::EDX,
            3 => Self::EBX,
            4 => Self::ESP,
            5 => Self::EBP,
            6 => Self::ESI,
            7 => Self::EDI,
            _ => panic!("index must be in range from 0 to 7"),
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_register(&self, reg: Register) -> u32 {
        match reg {
            Register::EAX => self.eax,
            Register::ECX => self.ecx,
            Register::EDX => self.edx,
            Register::EBX => self.ebx,
            Register::ESP => self.esp,
            Register::EBP => self.ebp,
            Register::ESI => self.esi,
            Register::EDI => self.edi,
            Register::EIP => self.eip,
        }
    }

    pub fn set_register(&mut self, reg: Register, value: u32) {
        match reg {
            Register::EAX => self.eax = value,
            Register::ECX => self.ecx = value,
            Register::EDX => self.edx = value,
            Register::EBX => self.ebx = value,
            Register::ESP => self.esp = value,
            Register::EBP => self.ebp = value,
            Register::ESI => self.esi = value,
            Register::EDI => self.edi = value,
            Register::EIP => self.eip = value,
        }
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        let bit = flag as u32;
        let value = self.flags & (1 << bit);
        return value != 0;
    }

    pub fn set_flag(&mut self, flag: Flags, value: bool) {
        let bit = flag as u32;
        if value {
            self.flags |= 1 << bit;
        } else {
            self.flags &= !(1 << bit);
        }
    }
}
