use x86asm::{decode, instruction::Instruction};

use crate::emulator::Emulator;

impl Emulator {
    pub fn decode(&mut self) -> Result<Instruction, String> {
        let rip = self.cpu.get_rip() as usize;
        let code = self.mmu.get_memory64(rip).to_le_bytes().to_vec();
        println!("Fetched: {:016X}", self.mmu.get_memory64(rip));

        let mut decoder = decode::Decoder::new(&code);
        let inst = decoder.decode_inst();
        let inst_len = decoder.pos();
        self.cpu.set_rip((rip + inst_len) as u64);

        Ok(inst)
    }
}
