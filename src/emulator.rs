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
    eip: u32,
}

impl Emulator {
    pub fn new(eip: u32, esp: u32) -> Self {
        let mut emu = Self {
            registers: [0; REGISTERS_COUNT],
            eflags: 0,
            memory: [0; MEMORY_SIZE],
            eip: eip,
        };
        emu.registers[ESP] = esp;

        return emu;
    }

    pub fn loadFromFile(&mut self, path: &str) {
        let mut file = File::open(path).expect("Failed to open file.");
        file.read(&mut self.memory[(self.eip as usize)..]).expect("Failed to read file.");
    }
}

