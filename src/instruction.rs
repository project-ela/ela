use crate::emulator::Emulator;

pub enum Opcode {
    MovR32Imm32(usize, u32),
    ShortJump(usize),
}

impl Emulator {
    pub fn decode(&mut self) -> Opcode {
        match self.get_code8(0) {
            0xB8..=0xBF => {
                let reg = self.get_code8(0) - 0xB8;
                let value = self.get_code32(1);
                self.eip += 5;
                Opcode::MovR32Imm32(reg as usize, value)
            }
            0xEB => {
                let diff = self.get_sign_code8(1);
                Opcode::ShortJump((diff + 2) as usize)
            }
            o => panic!("Not implemented: {:X}", o)
        }
    }

    pub fn exec(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::MovR32Imm32(reg, value) =>self.registers[reg] = value,
            Opcode::ShortJump(diff) => self.eip = self.eip.wrapping_add(diff),
        }
    }
}
