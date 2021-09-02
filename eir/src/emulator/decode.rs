use x86asm::{decode, instruction::Instruction};

use crate::emulator::Emulator;

impl Emulator {
    pub fn decode(&mut self) -> Result<Instruction, String> {
        let rip = self.cpu.get_rip() as usize;
        let mut bytes = Vec::new();
        for i in 0..8 {
            match self.mmu.get_memory8(rip + i) {
                Ok(value) => bytes.push(value),
                Err(_) => break,
            }
        }
        if self.dump_state {
            println!("Fetched: {:X?}", bytes);
        }

        let mut decoder = decode::Decoder::new(&bytes);
        let inst = decoder.decode_inst();
        let inst_len = decoder.pos();
        self.cpu.set_rip((rip + inst_len) as u64);

        Ok(inst)
    }
}
