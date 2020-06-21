use crate::emulator::Emulator;

#[derive(Debug)]
pub enum Opcode {
    // 89 /r
    MovRm32R32(usize, usize),
    // 8B /r
    MovR32Rm32(usize, usize),
    // B8+ rd id
    MovR32Imm32(usize, u32),
    // C7 /0 id
    MovRm32Imm32(usize, u32),
    // EB cb
    ShortJump(usize),
}

#[derive(Debug)]
pub struct ModRM {
    pub modval: u8,
    pub reg: u8,
    pub rm: u8,
}

impl Emulator {
    pub fn decode(&mut self) -> Opcode {
        match self.get_code8(0) {
            0x89 => {
                self.eip += 1;
                let modrm = self.parse_modrm();
                Opcode::MovRm32R32(modrm.rm as usize, modrm.reg as usize)
            }
            0x8B => {
                self.eip += 1;
                let modrm = self.parse_modrm();
                let disp = self.get_sign_code8(0);
                self.eip += 2;
                Opcode::MovR32Rm32(
                    modrm.reg as usize,
                    (self.registers[modrm.rm as usize] as i32 + disp as i32) as usize,
                )
            }
            0xB8..=0xBF => {
                let reg = self.get_code8(0) - 0xB8;
                let value = self.get_code32(1);
                self.eip += 5;
                Opcode::MovR32Imm32(reg as usize, value)
            }
            0xC7 => {
                self.eip += 1;
                let modrm = self.parse_modrm();
                let disp = self.get_sign_code8(0);
                let value = self.get_code8(1);
                self.eip += 5;
                Opcode::MovRm32Imm32(
                    (self.registers[modrm.reg as usize] as i32 + disp as i32) as usize,
                    value as u32,
                )
            }
            0xEB => {
                let diff = self.get_sign_code8(1);
                // add 2 because of 'cb'
                Opcode::ShortJump((diff + 2) as usize)
            }
            o => panic!("Not implemented: {:X}", o),
        }
    }

    pub fn exec(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::MovRm32R32(reg_to, reg_from) => {
                self.set_register(reg_to, self.get_register(reg_from))
            }
            Opcode::MovR32Rm32(reg, addr) => self.set_register(reg, self.get_memory32(addr)),
            Opcode::MovR32Imm32(reg, value) => self.set_register(reg, value),
            Opcode::MovRm32Imm32(addr, value) => self.set_memory32(addr, value),
            Opcode::ShortJump(diff) => self.eip = self.eip.wrapping_add(diff),
        }
    }

    fn parse_modrm(&mut self) -> ModRM {
        let code = self.get_code8(0);
        let modval = (code & 0b11000000) >> 6;
        let reg = (code & 0b00111000) >> 3;
        let rm = code & 0b00000111;
        self.eip += 1;
        ModRM { modval, reg, rm }
    }
}
