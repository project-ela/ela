pub mod cpu;
pub mod flags;
pub mod mmu;

use mmu::Mmu;

use crate::{
    emulator::cpu::{Cpu, Flags, Register, Register::*},
    instruction::modrm::RM,
};
use std::fs::File;
use std::io::Read;

const MEMORY_SIZE: usize = 1024 * 1024;

pub struct Emulator {
    pub cpu: Cpu,
    pub mmu: Mmu,

    // Size of binary
    len: u32,
    initial_eip: u32,
}

impl Emulator {
    pub fn new(eip: u32, esp: u32) -> Self {
        let mut emu = Self {
            cpu: Cpu::new(),
            mmu: Mmu::new(MEMORY_SIZE),
            len: 0,
            initial_eip: eip,
        };
        emu.cpu.set_register(EIP, eip);
        emu.cpu.set_register(ESP, esp);

        return emu;
    }

    pub fn load_from_file(&mut self, path: &str) {
        let mut file = File::open(path).expect("Failed to open file.");
        let eip = self.cpu.get_register(EIP) as usize;
        self.len = file
            .read(&mut self.mmu.get_raw_memory()[eip..])
            .expect("Failed to read file.") as u32;
        println!("Loaded {} bytes", self.len);
    }

    pub fn run(&mut self) {
        self.dump();
        while self.cpu.get_register(EIP) < self.initial_eip + self.len {
            match self.decode() {
                Ok(opcode) => {
                    println!("{:?}", opcode);
                    self.exec(opcode);
                }
                Err(err) => {
                    self.dump();
                    println!("Error: {}", err);
                    std::process::exit(1);
                }
            }
        }
    }

    pub fn inc_eip(&mut self, value: u32) {
        let eip = self.cpu.get_register(EIP);
        self.cpu.set_register(EIP, eip.wrapping_add(value));
    }

    pub fn get_code8(&self, index: usize) -> u8 {
        let eip = self.cpu.get_register(EIP) as usize;
        self.mmu.get_memory8(eip + index)
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

    pub fn get_rm(&self, rm: RM) -> u32 {
        match rm {
            RM::Register(reg) => self.cpu.get_register(reg),
            RM::Memory(addr) => self.mmu.get_memory32(addr),
        }
    }

    pub fn set_rm(&mut self, rm: RM, value: u32) {
        match rm {
            RM::Register(reg) => self.cpu.set_register(reg, value),
            RM::Memory(addr) => self.mmu.set_memory32(addr, value),
        }
    }

    pub fn push32(&mut self, value: u32) {
        let new_esp = self.cpu.get_register(ESP) - 4;
        self.mmu.set_memory32(new_esp as usize, value);
        self.cpu.set_register(ESP, new_esp);
    }

    pub fn pop32(&mut self) -> u32 {
        let esp = self.cpu.get_register(ESP);
        self.cpu.set_register(ESP, esp + 4);
        self.mmu.get_memory32(esp as usize)
    }

    pub fn dump(&self) {
        println!("----------------------------------------");
        println!("EIP: {:4X}", self.cpu.get_register(EIP));
        println!("Opcode: {:X}", self.get_code8(0));

        self.dump_eflags();
        self.dump_registers();
        self.dump_stack();
    }

    pub fn dump_eflags(&self) {
        println!(
            "flag: [Carry: {}, Zero: {}, Sign: {}, Overflow: {}]",
            self.cpu.get_flag(Flags::CF),
            self.cpu.get_flag(Flags::ZF),
            self.cpu.get_flag(Flags::SF),
            self.cpu.get_flag(Flags::OF),
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
            let esp = self.cpu.get_register(ESP) as usize;
            println!(
                "0x{:4X}: {:X}",
                esp + 4 * i,
                self.mmu.get_memory32(esp + 4 * i)
            );
        }
    }
}
