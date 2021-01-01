pub mod cpu;
pub mod decode;
pub mod execute;
pub mod flags;
pub mod mmu;

use std::fs;

use cpu::{Cpu, Flags};
use elfen::elf::Elf;
use mmu::Mmu;
use x86asm::instruction::operand::register::Register;

pub struct Emulator {
    pub cpu: Cpu,
    pub mmu: Mmu,
}

impl Emulator {
    pub fn new(rip: u64, rsp: u64) -> Self {
        let mut emu = Self {
            cpu: Cpu::new(),
            mmu: Mmu::new(),
        };
        emu.cpu.set_rip(rip);
        emu.cpu.set_register(&Register::Rsp, rsp);
        emu.mmu.add_segment(0, vec![0; rsp as usize]);

        return emu;
    }

    pub fn load_elf(&mut self, path: &str) {
        let file_data = fs::read(path).expect("Failed to read file.");
        let elf = Elf::read_from_file(path);

        for segment in &elf.segments {
            let offset = segment.offset as usize;
            let size = segment.file_size as usize;
            let virt_addr = segment.virt_addr as usize;
            let data = file_data[offset..(offset + size)].to_vec();
            self.mmu.add_segment(virt_addr, data);
        }

        let entrypoint = elf.header.entrypoint;
        self.cpu.set_rip(entrypoint);
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

    pub fn push64(&mut self, value: u64) -> Result<(), String> {
        let new_rsp = self.cpu.get_register(&Register::Rsp) - 8;
        self.mmu.set_memory64(new_rsp as usize, value)?;
        self.cpu.set_register(&Register::Rsp, new_rsp);
        Ok(())
    }

    pub fn pop64(&mut self) -> Result<u64, String> {
        let rsp = self.cpu.get_register(&Register::Rsp);
        self.cpu.set_register(&Register::Rsp, rsp + 8);
        self.mmu.get_memory64(rsp as usize)
    }

    pub fn dump(&self) {
        println!("----------------------------------------");
        println!("RIP: {:016X}", self.cpu.get_rip());

        self.dump_flags();
        self.dump_registers();
        self.dump_stack();

        println!();
    }

    pub fn dump_flags(&self) {
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
            let addr = rsp + 8 * i;
            match self.mmu.get_memory64(addr) {
                Ok(value) => println!("0x{:016X}: {:016X}", addr, value),
                Err(_) => break,
            }
        }
    }
}
