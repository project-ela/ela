use crate::emulator::Emulator;

impl Emulator {
    pub fn exec(&mut self) {
        let code = self.get_code8(0);
        match code {
            0xB8..=0xBF => self.mov_r32_imm32(),
            0xEB => self.short_jump(),
            _ => panic!("Not implemented: {:X}", code),
        }
        self.dump();
    }

    // MOV r32, imm32 | B8+rd id
    fn mov_r32_imm32(&mut self) {
        let reg = self.get_code8(0) - 0xB8;
        let value = self.get_code32(1);
        self.registers[reg as usize] = value;
        self.eip += 5;
    }

    // JMP rel8 | EB cb
    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1);
        // add 2 because of 'cb'
        self.eip = self.eip.wrapping_add((diff + 2) as usize);
    }
}