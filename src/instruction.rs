use crate::emulator::Emulator;

#[derive(Debug)]
pub enum Opcode {
    // 89 /r
    MovRm32R32(RM, usize),
    // 8B /r
    MovR32Rm32(usize, RM),
    // 90
    Nop,
    // B8+ rd id
    MovR32Imm32(usize, u32),
    // C7 /0 id
    MovRm32Imm32(RM, u32),
    // EB cb
    ShortJump(usize),
}

#[derive(Debug, Default)]
pub struct ModRM {
    pub modval: u8,
    pub reg: u8,
    pub rm: u8,

    pub disp8: i8,
    pub disp32: u32,
}

#[derive(Debug)]
pub enum RM {
    // index
    Register(usize),
    // address
    Memory(usize),
}

impl Emulator {
    pub fn decode(&mut self) -> Opcode {
        match self.get_code8(0) {
            0x89 => {
                self.eip += 1;
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::MovRm32R32(rm, modrm.reg as usize)
            }
            0x8B => {
                self.eip += 1;
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                self.eip += 2;
                Opcode::MovR32Rm32(modrm.reg as usize, rm)
            }
            0xB8..=0xBF => {
                let reg = self.get_code8(0) - 0xB8;
                let value = self.get_code32(1);
                self.eip += 5;
                Opcode::MovR32Imm32(reg as usize, value)
            }
            0x90 => {
                self.eip += 1;
                Opcode::Nop
            }
            0xC7 => {
                self.eip += 1;
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                let value = self.get_code8(1);
                self.eip += 4;
                Opcode::MovRm32Imm32(rm, value as u32)
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
            Opcode::MovRm32R32(rm, reg) => self.set_rm(rm, self.get_register(reg)),
            Opcode::MovR32Rm32(reg, rm) => self.set_register(reg, self.get_rm(rm)),
            Opcode::MovR32Imm32(reg, value) => self.set_register(reg, value),
            Opcode::Nop => {}
            Opcode::MovRm32Imm32(rm, value) => self.set_rm(rm, value),
            Opcode::ShortJump(diff) => self.eip = self.eip.wrapping_add(diff),
        }
    }

    fn parse_modrm(&mut self) -> ModRM {
        let mut modrm: ModRM = Default::default();
        let code = self.get_code8(0);
        modrm.modval = (code & 0b11000000) >> 6;
        modrm.reg = (code & 0b00111000) >> 3;
        modrm.rm = code & 0b00000111;
        self.eip += 1;

        if modrm.modval == 0b01 {
            modrm.disp8 = self.get_sign_code8(0);
            self.eip += 1;
        }

        return modrm;
    }

    pub fn calc_rm(&self, modrm: &ModRM) -> RM {
        match modrm.modval {
            0b01 => RM::Memory(
                (self.registers[modrm.reg as usize] as i32 + modrm.disp8 as i32) as usize,
            ),
            0b11 => RM::Register(modrm.rm as usize),
            x => panic!("Not implemented: {:X}", x),
        }
    }
}
