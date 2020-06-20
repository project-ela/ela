use crate::emulator::Emulator;

impl Emulator {
    pub fn exec(&mut self) {
        let code = self.get_code8(0);
        println!("eip: {}, opcode: {:X}", self.eip, code);
        match code {
            0xEB => self.short_jump(),
            _ => panic!("Not implemented: {:X}", code),
        }
    }

    // JMP rel8 | EB cb
    fn short_jump(&mut self) {
        let diff = self.get_sign_code8(1);
        // add 2 because of 'cb'
        self.eip = self.eip.wrapping_add((diff + 2) as usize);
    }
}