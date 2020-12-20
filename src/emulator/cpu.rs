use x86asm::instruction::operand::register::{self, Register};

#[derive(Debug, Default)]
pub struct Cpu {
    regs: [u64; 16],

    flags: u64,

    rip: u64,
}

pub enum Flags {
    CF = 0,
    PF = 2,
    ZF = 6,
    SF = 7,
    OF = 11,
}

impl Cpu {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_register(&self, reg: &Register) -> u64 {
        match reg.size() {
            register::Size::QWord | register::Size::DWord => self.get_register64(reg),
            register::Size::Byte => self.get_register8(reg) as u64,
            _ => unimplemented!(),
        }
    }

    pub fn set_register(&mut self, reg: &Register, value: u64) {
        match reg.size() {
            register::Size::QWord | register::Size::DWord => self.set_register64(reg, value),
            register::Size::Byte => self.set_register8(reg, value as u8),
            _ => unimplemented!(),
        }
    }

    pub fn get_register8(&self, reg: &Register) -> u8 {
        self.regs[reg_num(reg)] as u8
    }

    pub fn set_register8(&mut self, reg: &Register, value: u8) {
        self.regs[reg_num(reg)] = (self.regs[reg_num(reg)] & 0xffffff00) | value as u64;
    }

    pub fn get_register64(&self, reg: &Register) -> u64 {
        self.regs[reg_num(reg)]
    }

    pub fn set_register64(&mut self, reg: &Register, value: u64) {
        self.regs[reg_num(reg)] = value;
    }

    pub fn get_rip(&self) -> u64 {
        self.rip
    }

    pub fn set_rip(&mut self, value: u64) {
        self.rip = value;
    }

    pub fn get_flag(&self, flag: Flags) -> bool {
        let bit = flag as u32;
        let value = self.flags & (1 << bit);
        value != 0
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

fn reg_num(reg: &Register) -> usize {
    let extend = reg.only_in_64bit();
    reg.number() as usize + if extend { 8 } else { 0 }
}
