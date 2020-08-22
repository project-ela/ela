use crate::cpu::Register::*;
use crate::cpu::{Register, CPU, EFLAGS};
use crate::instruction::RM;
use std::fs::File;
use std::io::Read;

const MEMORY_SIZE: usize = 1024 * 1024;

pub struct Emulator {
    cpu: CPU,
    memory: Vec<u8>,

    // Size of binary
    len: u32,
    initial_eip: u32,
}

impl Emulator {
    pub fn new(eip: u32, esp: u32) -> Self {
        let mut emu = Self {
            cpu: CPU::new(),
            memory: vec![0; MEMORY_SIZE],
            len: 0,
            initial_eip: eip,
        };
        emu.set_register(EIP, eip);
        emu.set_register(ESP, esp);

        return emu;
    }

    pub fn load_from_file(&mut self, path: &str) {
        let mut file = File::open(path).expect("Failed to open file.");
        let eip = self.get_register(EIP) as usize;
        self.len = file
            .read(&mut self.memory[eip..])
            .expect("Failed to read file.") as u32;
        println!("Loaded {} bytes", self.len);
    }

    pub fn run(&mut self) {
        self.dump();
        while self.get_register(EIP) < self.initial_eip + self.len {
            let opcode = self.decode();
            self.exec(opcode);
            self.dump();
        }
    }

    pub fn inc_eip(&mut self, value: u32) {
        let eip = self.get_register(EIP);
        self.set_register(EIP, eip.wrapping_add(value));
    }

    pub fn get_code8(&self, index: usize) -> u8 {
        let eip = self.get_register(EIP) as usize;
        self.memory[eip + index]
    }

    pub fn get_sign_code8(&self, index: usize) -> i8 {
        self.get_code8(index) as i8
    }

    pub fn get_code32(&self, index: usize) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.get_code8(index + i) as u32) << (i * 8)
        }
        return ret;
    }

    pub fn get_sign_code32(&self, index: usize) -> i32 {
        self.get_code32(index) as i32
    }

    pub fn get_register(&self, reg: Register) -> u32 {
        self.cpu.get_register(reg)
    }

    pub fn set_register(&mut self, reg: Register, value: u32) {
        self.cpu.set_register(reg, value);
    }

    pub fn get_memory8(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn get_memory32(&self, address: usize) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.get_memory8(address + i) as u32) >> (8 * i);
        }
        return ret;
    }

    pub fn set_memory8(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn set_memory32(&mut self, address: usize, value: u32) {
        for i in 0..4 {
            self.set_memory8(address + i, (value << (8 * i)) as u8);
        }
    }

    pub fn get_rm(&self, rm: RM) -> u32 {
        match rm {
            RM::Register(reg) => self.get_register(reg),
            RM::Memory(addr) => self.get_memory32(addr),
        }
    }

    pub fn set_rm(&mut self, rm: RM, value: u32) {
        match rm {
            RM::Register(reg) => self.set_register(reg, value),
            RM::Memory(addr) => self.set_memory32(addr, value),
        }
    }

    pub fn push32(&mut self, value: u32) {
        let new_esp = self.get_register(ESP) - 4;
        self.set_memory32(new_esp as usize, value);
        self.set_register(ESP, new_esp);
    }

    pub fn pop32(&mut self) -> u32 {
        let esp = self.get_register(ESP);
        self.set_register(ESP, esp + 4);
        self.get_memory32(esp as usize)
    }

    pub fn update_eflags_add(&mut self, lhs: u32, rhs: u32, result: u64) {
        self.update_eflags_sub(lhs, rhs, result);
    }

    pub fn update_eflags_sub(&mut self, lhs: u32, rhs: u32, result: u64) {
        let lhs_sign = lhs >> 31;
        let rhs_sign = rhs >> 31;
        let result_sign = (result >> 31) as u32;
        self.cpu.set_eflag(EFLAGS::CF, (result >> 32) != 0);
        self.cpu.set_eflag(EFLAGS::ZF, result == 0);
        self.cpu.set_eflag(EFLAGS::SF, result_sign != 0);
        self.cpu.set_eflag(
            EFLAGS::OF,
            (lhs_sign != rhs_sign) && (lhs_sign != result_sign),
        );
    }

    pub fn update_eflags_xor(&mut self, result: u64) {
        let result_sign = (result >> 31) as u32;
        self.cpu.set_eflag(EFLAGS::CF, false);
        self.cpu.set_eflag(EFLAGS::ZF, result != 0);
        self.cpu.set_eflag(EFLAGS::SF, result_sign != 0);
        self.cpu.set_eflag(EFLAGS::OF, false);
    }

    pub fn dump(&self) {
        println!("----------------------------------------");
        println!("EIP: {:4X}", self.get_register(EIP));
        println!("Opcode: {:X}", self.get_code8(0));

        self.dump_eflags();
        self.dump_registers();
        self.dump_stack();
    }

    pub fn dump_eflags(&self) {
        println!(
            "flag: [Carry: {}, Zero: {}, Sign: {}, Overflow: {}]",
            self.cpu.get_eflag(EFLAGS::CF),
            self.cpu.get_eflag(EFLAGS::ZF),
            self.cpu.get_eflag(EFLAGS::SF),
            self.cpu.get_eflag(EFLAGS::OF),
        );
    }

    pub fn dump_registers(&self) {
        print!("EAX: {:4X}, ", self.cpu.get_register(Register::EAX));
        print!("ECX: {:4X}, ", self.cpu.get_register(Register::ECX));
        print!("EDX: {:4X}, ", self.cpu.get_register(Register::EDX));
        println!("EBX: {:4X}, ", self.cpu.get_register(Register::EBX));
        print!("ESP: {:4X}, ", self.cpu.get_register(Register::ESP));
        print!("EBP: {:4X}, ", self.cpu.get_register(Register::EBP));
        print!("ESI: {:4X}, ", self.cpu.get_register(Register::ESI));
        println!("EDI: {:4X}, ", self.cpu.get_register(Register::EDI));
    }

    pub fn dump_stack(&self) {
        println!("----- stack -----");
        for i in 0..5 {
            let esp = self.get_register(ESP) as usize;
            println!("0x{:4X}: {:X}", esp + 4 * i, self.get_memory32(esp + 4 * i));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_eflags_sub() {
        let mut emu = Emulator::new(0x7c00, 0x7c00);
        emu.update_eflags_sub(10, 5, 5);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::CF), false);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::ZF), false);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::SF), false);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::OF), false);

        emu.update_eflags_sub(5, 10, (-5 as i64) as u64);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::CF), true);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::ZF), false);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::SF), true);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::OF), false);

        emu.update_eflags_sub(10, 10, 0);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::CF), false);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::ZF), true);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::SF), false);
        assert_eq!(emu.cpu.get_eflag(EFLAGS::OF), false);
    }
}
