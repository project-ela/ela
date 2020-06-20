use std::fs::File;
use std::io::Read;

const MEMORY_SIZE: usize = 1024 * 1024;
const REGISTERS_COUNT: usize = 8;

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
    registers: [u32; REGISTERS_COUNT],
    eflags: u32,
    memory: [u8; MEMORY_SIZE],

    // Instruction Pointer
    pub eip: usize,

    // Size of binary
    len: usize,
}

impl Emulator {
    pub fn new(eip: usize, esp: u32) -> Self {
        let mut emu = Self {
            registers: [0; REGISTERS_COUNT],
            eflags: 0,
            memory: [0; MEMORY_SIZE],
            eip: eip,
            len: 0,
        };
        emu.registers[ESP] = esp;

        return emu;
    }

    pub fn load_from_file(&mut self, path: &str) {
        let mut file = File::open(path).expect("Failed to open file.");
        self.len = file.read(&mut self.memory[(self.eip as usize)..]).expect("Failed to read file.");
    }

    pub fn run(&mut self) {
        while self.eip < self.len {
            self.exec();
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
}

