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

    eflags: u32,

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

pub enum EFLAGS {
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

    pub fn get_eflag(&self, flag: EFLAGS) -> bool {
        let bit = flag as u32;
        let value = self.eflags & (1 << bit);
        return value != 0;
    }

    pub fn set_eflag(&mut self, flag: EFLAGS, value: bool) {
        let bit = flag as u32;
        if value {
            self.eflags |= 1 << bit;
        } else {
            self.eflags &= !(1 << bit);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_register(cpu: &Cpu, expected: [u32; 8]) {
        assert_eq!(cpu.get_register(Register::EAX), expected[0]);
        assert_eq!(cpu.get_register(Register::ECX), expected[1]);
        assert_eq!(cpu.get_register(Register::EDX), expected[2]);
        assert_eq!(cpu.get_register(Register::EBX), expected[3]);
        assert_eq!(cpu.get_register(Register::ESP), expected[4]);
        assert_eq!(cpu.get_register(Register::EBP), expected[5]);
        assert_eq!(cpu.get_register(Register::ESI), expected[6]);
        assert_eq!(cpu.get_register(Register::EDI), expected[7]);
    }

    #[test]
    fn registers() {
        let mut cpu = Cpu::new();
        test_register(&cpu, [0, 0, 0, 0, 0, 0, 0, 0]);

        cpu.set_register(Register::EAX, 0x42);
        test_register(&cpu, [0x42, 0, 0, 0, 0, 0, 0, 0]);

        cpu.set_register(Register::ECX, 0xdeadbeef);
        test_register(&cpu, [0x42, 0xdeadbeef, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn eflags() {
        let mut cpu = Cpu::new();
        assert_eq!(cpu.eflags, 0b00000000000000000000000000000000);

        cpu.set_eflag(EFLAGS::SF, true);
        assert_eq!(cpu.eflags, 0b00000000000000000000000010000000);
        assert_eq!(cpu.get_eflag(EFLAGS::SF), true);

        cpu.set_eflag(EFLAGS::CF, true);
        assert_eq!(cpu.eflags, 0b00000000000000000000000010000001);
        assert_eq!(cpu.get_eflag(EFLAGS::CF), true);

        cpu.set_eflag(EFLAGS::SF, false);
        assert_eq!(cpu.eflags, 0b00000000000000000000000000000001);
        assert_eq!(cpu.get_eflag(EFLAGS::SF), false);
    }
}
