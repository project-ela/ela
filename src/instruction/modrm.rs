use crate::emulator::{cpu::Register, Emulator};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ModRM {
    pub modval: u8,
    pub reg: u8,
    pub rm: u8,

    pub disp8: i8,
    pub disp32: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RM {
    // index
    Register(Register),
    // address
    Memory(usize),
}

impl Emulator {
    pub fn parse_modrm(&mut self) -> ModRM {
        let mut modrm: ModRM = Default::default();
        let code = self.get_code8(0);
        modrm.modval = (code & 0b11000000) >> 6;
        modrm.reg = (code & 0b00111000) >> 3;
        modrm.rm = code & 0b00000111;
        self.inc_eip(1);

        if modrm.modval == 0b01 {
            modrm.disp8 = self.get_sign_code8(0);
            self.inc_eip(1);
        }

        return modrm;
    }

    pub fn calc_rm(&self, modrm: &ModRM) -> RM {
        match modrm.modval {
            0b00 => {
                let reg = Register::from(modrm.rm);
                let reg_value = self.cpu.get_register(reg);
                RM::Memory(reg_value as usize)
            }
            0b01 => {
                let reg = Register::from(modrm.rm);
                let reg_value = self.cpu.get_register(reg);
                RM::Memory((reg_value as i32 + modrm.disp8 as i32) as usize)
            }
            0b11 => RM::Register(Register::from(modrm.rm)),
            x => panic!("Not implemented: {:X}", x),
        }
    }
}
