use crate::instruction::RM;
use std::fs::File;
use std::io::Read;

const MEMORY_SIZE: usize = 1024 * 1024;
const REGISTERS_COUNT: usize = 8;
const REGISTERS_NAME: [&str; 8] = ["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];

// Accumulator
pub const EAX: usize = 0;
// Counter
pub const ECX: usize = 1;
// Data
pub const EDX: usize = 2;
// Base
pub const EBX: usize = 3;
// Stack Pointer
pub const ESP: usize = 4;
// Stack Base Pointer
pub const EBP: usize = 5;
// Source Index
pub const ESI: usize = 6;
// Destination Index
pub const EDI: usize = 7;

pub struct Emulator {
    pub registers: [u32; REGISTERS_COUNT],
    pub eflags: u32,
    pub memory: Vec<u8>,

    // Instruction Pointer
    pub eip: usize,
    initial_eip: usize,

    // Size of binary
    pub len: usize,
}

impl Emulator {
    pub fn new(eip: usize, esp: u32) -> Self {
        let mut emu = Self {
            registers: [0; REGISTERS_COUNT],
            eflags: 0,
            memory: vec![0; MEMORY_SIZE],
            eip: eip,
            initial_eip: eip,
            len: 0,
        };
        emu.registers[ESP] = esp;

        return emu;
    }

    pub fn load_from_file(&mut self, path: &str) {
        let mut file = File::open(path).expect("Failed to open file.");
        self.len = file
            .read(&mut self.memory[(self.eip as usize)..])
            .expect("Failed to read file.");
        println!("Loaded {} bytes", self.len);
    }

    pub fn run(&mut self) {
        self.dump();
        while self.eip < self.initial_eip + self.len {
            let opcode = self.decode();
            self.exec(opcode);
            self.dump();
        }
    }

    pub fn get_code8(&self, index: usize) -> u8 {
        self.memory[self.eip + index]
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

    pub fn get_register(&self, index: usize) -> u32 {
        self.registers[index]
    }

    pub fn set_register(&mut self, index: usize, value: u32) {
        self.registers[index] = value;
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
            RM::Register(index) => self.get_register(index),
            RM::Memory(addr) => self.get_memory32(addr),
        }
    }

    pub fn set_rm(&mut self, rm: RM, value: u32) {
        match rm {
            RM::Register(index) => self.set_register(index, value),
            RM::Memory(addr) => self.set_memory32(addr, value),
        }
    }

    pub fn dump(&self) {
        println!("----------------------------------------");
        println!("EIP: {:4X}", self.eip);
        println!("Opcode: {:X}", self.get_code8(0));

        self.dump_registers();
    }

    pub fn dump_registers(&self) {
        for i in 0..REGISTERS_COUNT {
            if i != 0 {
                print!(", ");
                if i % 4 == 0 {
                    println!("");
                }
            }

            print!("{}: {:4X}", REGISTERS_NAME[i], self.registers[i]);
        }

        println!("");
    }
}
