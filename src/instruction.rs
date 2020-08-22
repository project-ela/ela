use crate::cpu::Register;
use crate::emulator::Emulator;

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    // 01 /r
    AddRm32R32(RM, Register),
    // 29 /r
    SubRm32R32(RM, Register),
    // 2B /r
    SubR32Rm32(Register, RM),
    // 31 /r
    XorR32Rm32(Register, RM),
    // 50+rd
    PushR32(Register),
    // 58
    PopR32(Register),
    // 6A ib
    PushImm8(u32),
    // 81 /5 id
    SubRm32Imm32(RM, u32),
    // 83 /0 ib
    AddRm32Imm32(RM, u32),
    // 83 /6 ib0xc5c9799fL
    XorRm32Imm32(RM, u32),
    // 89 /r
    MovRm32R32(RM, Register),
    // 8B /r
    MovR32Rm32(Register, RM),
    // 8F /0
    PopRm32(RM),
    // 90
    Nop,
    // B8+ rd id
    MovR32Imm32(Register, u32),
    // C7 /0 id
    MovRm32Imm32(RM, u32),
    // EB cb
    ShortJump(u8),
    // F7 /5
    IMulRm32(RM),
    // F7 /7
    IDivRm32(RM),
    // FF /6
    PushRm32(RM),
}

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
    pub fn decode(&mut self) -> Opcode {
        match self.get_code8(0) {
            0x01 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::AddRm32R32(rm, Register::from(modrm.reg))
            }
            0x29 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::SubRm32R32(rm, Register::from(modrm.reg))
            }
            0x2B => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::SubR32Rm32(Register::from(modrm.reg), rm)
            }
            0x31 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::XorR32Rm32(Register::from(modrm.reg), rm)
            }
            0x50..=0x57 => {
                let reg = self.get_code8(0) - 0x50;
                self.inc_eip(1);
                Opcode::PushR32(Register::from(reg))
            }
            0x58..=0x5F => {
                let reg = self.get_code8(0) - 0x58;
                self.inc_eip(1);
                Opcode::PopR32(Register::from(reg))
            }
            0x6A => {
                let value = self.get_code8(1);
                self.inc_eip(2);
                Opcode::PushImm8(value as u32)
            }
            0x81 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                let value = self.get_code8(0);
                Opcode::SubRm32Imm32(rm, value as u32)
            }
            0x83 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                let value = self.get_code8(0);
                match modrm.reg {
                    0b000 => Opcode::AddRm32Imm32(rm, value as u32),
                    0b110 => Opcode::XorRm32Imm32(rm, value as u32),
                    o => panic!("Not implemented: {:X}", o),
                }
            }
            0x89 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::MovRm32R32(rm, Register::from(modrm.reg))
            }
            0x8B => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::MovR32Rm32(Register::from(modrm.reg), rm)
            }
            0x8F => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::PopRm32(rm)
            }
            0x90 => {
                self.inc_eip(1);
                Opcode::Nop
            }
            0xB8..=0xBF => {
                let reg = self.get_code8(0) - 0xB8;
                let value = self.get_code32(1);
                self.inc_eip(5);
                Opcode::MovR32Imm32(Register::from(reg), value)
            }
            0xC7 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                let value = self.get_code8(0);
                self.inc_eip(4);
                Opcode::MovRm32Imm32(rm, value as u32)
            }
            0xEB => {
                let diff = self.get_code8(1);
                // add 2 because of 'cb'
                Opcode::ShortJump(diff + 2)
            }
            0xF7 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                match modrm.reg {
                    0b101 => Opcode::IMulRm32(rm),
                    0b111 => Opcode::IDivRm32(rm),
                    o => panic!("Not implemented: {:X}", o),
            }
            }
            0xFF => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::PushRm32(rm)
            }
            o => panic!("Not implemented: {:X}", o),
        }
    }

    pub fn exec(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::AddRm32R32(rm, reg) => {
                let lhs = self.get_rm(rm);
                let rhs = self.get_register(reg);
                let result = (lhs as u64).wrapping_add(rhs as u64);
                self.update_eflags_add(lhs, rhs, result);
                self.set_rm(rm, result as u32);
            }
            Opcode::SubRm32R32(rm, reg) => {
                let lhs = self.get_rm(rm);
                let rhs = self.get_register(reg);
                let result = (lhs as u64).wrapping_sub(rhs as u64);
                self.update_eflags_sub(lhs, rhs, result);
                self.set_rm(rm, result as u32);
            }
            Opcode::SubR32Rm32(reg, rm) => {
                let lhs = self.get_register(reg);
                let rhs = self.get_rm(rm);
                let result = (lhs as u64).wrapping_sub(rhs as u64);
                self.update_eflags_sub(lhs, rhs, result);
                self.set_register(reg, result as u32);
            }
            Opcode::XorR32Rm32(reg, rm) => {
                let lhs = self.get_register(reg);
                let rhs = self.get_rm(rm);
                let result = lhs ^ rhs;
                self.update_eflags_xor(result as u64);
                self.set_register(reg, result);
            }
            Opcode::PushR32(reg) => self.push32(self.get_register(reg)),
            Opcode::PopR32(reg) => {
                let value = self.pop32();
                self.set_register(reg, value);
            }
            Opcode::PushImm8(value) => self.push32(value),
            Opcode::SubRm32Imm32(rm, value) => {
                let lhs = self.get_rm(rm);
                let result = (lhs as u64).wrapping_sub(value as u64);
                self.update_eflags_sub(lhs, value, result);
                self.set_rm(rm, result as u32);
            }
            Opcode::AddRm32Imm32(rm, value) => {
                let lhs = self.get_rm(rm);
                let result = (lhs as u64).wrapping_add(value as u64);
                self.update_eflags_add(lhs, value, result);
                self.set_rm(rm, result as u32);
            }
            Opcode::XorRm32Imm32(rm, value) => {
                let lhs = self.get_rm(rm);
                let result = lhs ^ value;
                self.update_eflags_xor(result as u64);
                self.set_rm(rm, result);
            }
            Opcode::MovRm32R32(rm, reg) => self.set_rm(rm, self.get_register(reg)),
            Opcode::MovR32Rm32(reg, rm) => self.set_register(reg, self.get_rm(rm)),
            Opcode::PopRm32(rm) => {
                let value = self.pop32();
                self.set_rm(rm, value);
            }
            Opcode::Nop => {}
            Opcode::MovR32Imm32(reg, value) => self.set_register(reg, value),
            Opcode::MovRm32Imm32(rm, value) => self.set_rm(rm, value),
            Opcode::ShortJump(diff) => self.inc_eip(diff as u32),
            Opcode::IMulRm32(rm) => {
                let lhs = self.get_register(Register::EAX);
                let rhs = self.get_rm(rm);
                let result = lhs.wrapping_mul(rhs);
                //TODO: update eflags
                self.set_register(Register::EAX, result);
            }
            Opcode::IDivRm32(rm) => {
                let lhs = self.get_register(Register::EAX);
                let rhs = self.get_rm(rm);
                let result = lhs.wrapping_div(rhs);
                //TODO: update eflags
                self.set_register(Register::EAX, result);
            }
            Opcode::PushRm32(rm) => self.push32(self.get_rm(rm)),
        }
    }

    fn parse_modrm(&mut self) -> ModRM {
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
            0b01 => {
                let reg = Register::from(modrm.rm);
                let reg_value = self.get_register(reg);
                RM::Memory((reg_value as i32 + modrm.disp8 as i32) as usize)
            }
            0b11 => RM::Register(Register::from(modrm.rm)),
            x => panic!("Not implemented: {:X}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Opcode, RM};
    use crate::cpu::Register::*;
    use crate::emulator::*;

    fn new_emu(bin: &[u8]) -> Emulator {
        let mut emu = Emulator::new(0x7C00, 0x7C00);
        for (i, v) in bin.iter().enumerate() {
            emu.set_memory8(0x7C00 + i, *v);
        }
        return emu;
    }

    #[test]
    fn push_r32() {
        let mut emu = new_emu(&[0x50]);
        emu.set_register(EAX, 0x20);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x1);
        assert_eq!(opcode, Opcode::PushR32(EAX));
        emu.exec(opcode);
        assert_eq!(
            emu.get_memory32(emu.get_register(ESP) as usize),
            emu.get_register(EAX)
        );
    }

    #[test]
    fn pop_r32() {
        let mut emu = new_emu(&[0x58]);
        emu.push32(0xB);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x1);
        assert_eq!(opcode, Opcode::PopR32(EAX));
        emu.exec(opcode);
        assert_eq!(emu.get_register(EAX), 0xB);
    }

    #[test]
    fn push_imm8() {
        let mut emu = new_emu(&[0x6A, 0x0A]);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x2);
        assert_eq!(opcode, Opcode::PushImm8(0xA));
        emu.exec(opcode);
        assert_eq!(emu.get_memory32(emu.get_register(ESP) as usize), 0xA);
    }

    #[test]
    fn mov_rm32_r32() {
        let mut emu = new_emu(&[0x89, 0xE5, 0x89, 0x45, 0xFC]);

        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x2);
        assert_eq!(opcode, Opcode::MovRm32R32(RM::Register(EBP), ESP));
        emu.exec(opcode);
        assert_eq!(emu.get_register(EBP), emu.get_register(ESP));

        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x5);
        let addr = (emu.get_register(EBP) as i32 - 4 as i32) as usize;
        assert_eq!(opcode, Opcode::MovRm32R32(RM::Memory(addr), EAX));
        emu.exec(opcode);
        assert_eq!(emu.get_memory32(addr), emu.get_register(EAX));
    }

    #[test]
    fn mov_r32_rm32() {
        let mut emu = new_emu(&[0x8B, 0x45, 0xFC]);
        emu.set_register(EBP, 0x7C00);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x3);
        let addr = (emu.get_register(EBP) as i32 - 4 as i32) as usize;
        assert_eq!(opcode, Opcode::MovR32Rm32(EAX, RM::Memory(addr)));
        emu.exec(opcode);
        assert_eq!(emu.get_register(EAX), emu.get_memory32(addr));
    }

    #[test]
    fn pop_rm32() {
        let mut emu = new_emu(&[0x8F, 0x45, 0x04]);
        emu.push32(0xB);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x3);
        let addr = (emu.get_register(EBP) as i32 + 4 as i32) as usize;
        assert_eq!(opcode, Opcode::PopRm32(RM::Memory(addr)));
        emu.exec(opcode);
        assert_eq!(emu.get_memory32(addr), 0xB);
    }

    #[test]
    fn nop() {
        let mut emu = new_emu(&[0x90]);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x1);
        assert_eq!(opcode, Opcode::Nop);
        emu.exec(opcode);
    }

    #[test]
    fn mov_r32_imm32() {
        let mut emu = new_emu(&[0xB8, 0x41, 0x00, 0x00, 0x00]);
        let opcode = emu.decode();
        assert_eq!(opcode, Opcode::MovR32Imm32(EAX, 0x41));
        emu.exec(opcode);
        assert_eq!(emu.get_register(EAX), 0x41);
    }

    #[test]
    fn mov_rm32_imm32() {
        let mut emu = new_emu(&[0xC7, 0x45, 0xFC, 0x0A, 0x00, 0x00, 0x00]);
        emu.set_register(EBP, 0x7C00);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x7);
        let addr = (emu.get_register(EBP) as i32 - 4 as i32) as usize;
        assert_eq!(opcode, Opcode::MovRm32Imm32(RM::Memory(addr), 0xA));
        emu.exec(opcode);
        assert_eq!(emu.get_memory32(addr), 0xA);
    }

    #[test]
    fn short_jump() {
        let mut emu = new_emu(&[0xEB, 0x76]);
        let opcode = emu.decode();
        assert_eq!(opcode, Opcode::ShortJump(0x76 + 0x2));
        emu.exec(opcode);
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x76 + 0x2);
    }

    #[test]
    fn push_rm32() {
        let mut emu = new_emu(&[0xFF, 0x75, 0x04]);
        emu.set_register(EBP, 0x7C00);
        let addr = (emu.get_register(EBP) as i32 + 4 as i32) as usize;
        emu.set_memory32(addr, 0xA);
        let opcode = emu.decode();
        assert_eq!(emu.get_register(EIP), 0x7C00 + 0x3);
        assert_eq!(opcode, Opcode::PushRm32(RM::Memory(addr)));
        emu.exec(opcode);
        assert_eq!(emu.get_memory32(emu.get_register(ESP) as usize), 0xA);
    }
}
