pub mod cpu;
pub mod decode;
pub mod execute;
pub mod flags;
pub mod mmu;

use std::fs::File;
use std::io::Read;

use cpu::{Cpu, Flags};
use mmu::Mmu;
use x86asm::instruction::operand::register::Register;

const MEMORY_SIZE: usize = 1024 * 1024;

pub struct Emulator {
    pub cpu: Cpu,
    pub mmu: Mmu,
}

impl Emulator {
    pub fn new(rip: u64, rsp: u64) -> Self {
        let mut emu = Self {
            cpu: Cpu::new(),
            mmu: Mmu::new(MEMORY_SIZE),
        };
        emu.cpu.set_rip(rip);
        emu.cpu.set_register(&Register::Rsp, rsp);

        return emu;
    }

    pub fn load_from_file(&mut self, path: &str) {
        let mut file = File::open(path).expect("Failed to open file.");
        let rip = self.cpu.get_rip() as usize;
        let len = file
            .read(&mut self.mmu.get_raw_memory()[rip..])
            .expect("Failed to read file.") as u32;
        println!("Loaded {} bytes", len);
    }

    pub fn run(&mut self) {
        self.dump();
        loop {
            match self.decode() {
                Ok(inst) => {
                    println!("Decoded: {:?}", inst);
                    self.exec(inst);
                    self.dump();
                }
                Err(err) => {
                    self.dump();
                    println!("Error: {}", err);
                    std::process::exit(1);
                }
            }
        }
    }

    pub fn inc_eip(&mut self, value: u64) {
        let rip = self.cpu.get_rip();
        self.cpu.set_rip(rip.wrapping_add(value));
    }

    pub fn get_code8(&self, index: usize) -> u8 {
        let rip = self.cpu.get_rip() as usize;
        self.mmu.get_memory8(rip + index)
    }

    pub fn get_code32(&self, index: usize) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.get_code8(index + i) as u32) << (i * 8)
        }
        return ret;
    }

    pub fn push64(&mut self, value: u64) {
        let new_rsp = self.cpu.get_register(&Register::Rsp) - 8;
        self.mmu.set_memory64(new_rsp as usize, value);
        self.cpu.set_register(&Register::Rsp, new_rsp);
    }

    pub fn pop64(&mut self) -> u64 {
        let rsp = self.cpu.get_register(&Register::Rsp);
        self.cpu.set_register(&Register::Rsp, rsp + 8);
        self.mmu.get_memory64(rsp as usize)
    }

    pub fn dump(&self) {
        println!("----------------------------------------");
        println!("RIP: {:8X}", self.cpu.get_rip());
        println!("Opcode: {:X}", self.get_code8(0));

        self.dump_eflags();
        self.dump_registers();
        self.dump_stack();

        println!();
    }

    pub fn dump_eflags(&self) {
        println!(
            "flag: [Carry: {}, Parity: {}, Zero: {}, Sign: {}, Overflow: {}]",
            self.cpu.get_flag(Flags::CF),
            self.cpu.get_flag(Flags::PF),
            self.cpu.get_flag(Flags::ZF),
            self.cpu.get_flag(Flags::SF),
            self.cpu.get_flag(Flags::OF),
        );
    }

    pub fn dump_registers(&self) {
        print!("RAX: {:016X}, ", self.cpu.get_register(&Register::Rax));
        print!("RCX: {:016X}, ", self.cpu.get_register(&Register::Rcx));
        print!("RDX: {:016X}, ", self.cpu.get_register(&Register::Rdx));
        println!("RBX: {:016X}, ", self.cpu.get_register(&Register::Rbx));
        print!("RSP: {:016X}, ", self.cpu.get_register(&Register::Rsp));
        print!("RBP: {:016X}, ", self.cpu.get_register(&Register::Rbp));
        print!("RSI: {:016X}, ", self.cpu.get_register(&Register::Rsi));
        println!("RDI: {:016X}, ", self.cpu.get_register(&Register::Rdi));
        print!("R8 : {:016X}, ", self.cpu.get_register(&Register::R8));
        print!("R9 : {:016X}, ", self.cpu.get_register(&Register::R9));
        print!("R10: {:016X}, ", self.cpu.get_register(&Register::R10));
        println!("R11: {:016X}, ", self.cpu.get_register(&Register::R11));
        print!("R12: {:016X}, ", self.cpu.get_register(&Register::R12));
        print!("R13: {:016X}, ", self.cpu.get_register(&Register::R13));
        print!("R14: {:016X}, ", self.cpu.get_register(&Register::R14));
        println!("R15: {:016X}, ", self.cpu.get_register(&Register::R15));
    }

    pub fn dump_stack(&self) {
        println!("----- stack -----");
        for i in 0..5 {
            let rsp = self.cpu.get_register(&Register::Rsp) as usize;
            println!(
                "0x{:016X}: {:016X}",
                rsp + 8 * i,
                self.mmu.get_memory64(rsp + 8 * i)
            );
        }
    }
}
