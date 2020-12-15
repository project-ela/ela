use crate::emulator::{cpu::Register, Emulator};
use crate::instruction::Opcode;

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
            0xC3 => Opcode::Ret,
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
}
