pub struct Mmu {
    ram: Vec<u8>,
}

impl Mmu {
    pub fn new(ram_size: usize) -> Self {
        Self {
            ram: vec![0; ram_size],
        }
    }

    pub fn get_memory8(&self, address: usize) -> u8 {
        self.ram[address]
    }

    pub fn get_memory32(&self, address: usize) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.get_memory8(address + i) as u32) << (8 * i);
        }
        return ret;
    }

    pub fn get_memory64(&self, address: usize) -> u64 {
        let mut ret = 0;
        for i in 0..8 {
            ret |= (self.get_memory8(address + i) as u64) << (8 * i);
        }
        return ret;
    }

    pub fn set_memory8(&mut self, address: usize, value: u8) {
        self.ram[address] = value;
    }

    pub fn set_memory32(&mut self, address: usize, value: u32) {
        for i in 0..4 {
            self.set_memory8(address + i, (value >> (8 * i)) as u8);
        }
    }

    pub fn set_memory64(&mut self, address: usize, value: u64) {
        for i in 0..8 {
            self.set_memory8(address + i, (value >> (8 * i)) as u8);
        }
    }

    pub fn get_raw_memory(&mut self) -> &mut Vec<u8> {
        &mut self.ram
    }
}
