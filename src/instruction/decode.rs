use crate::{
    emulator::{cpu::Register, Emulator},
    instruction::{modrm::RM, Opcode},
};

impl Emulator {
    pub fn decode(&mut self) -> Result<Opcode, String> {
        Ok(match self.get_code8(0) {
            0x00 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                let reg = self.get_code8(0);
                self.inc_eip(1);
                Opcode::AddRm8R8(rm, Register::from(reg))
            }
            0x01 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::AddRm32R32(rm, Register::from(modrm.reg))
            }
            0x0F => {
                let op = self.get_code8(1);
                self.inc_eip(2);
                match op {
                    0x84 => {
                        let diff = self.get_code32(0);
                        self.inc_eip(4);
                        return Ok(Opcode::Jz32(diff));
                    }
                    _ => {}
                }
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                match op {
                    0x94 => Opcode::SetE(rm),
                    0x95 => Opcode::SetNE(rm),
                    0x9C => Opcode::SetL(rm),
                    0x9D => Opcode::SetGE(rm),
                    0x9E => Opcode::SetLE(rm),
                    0x9F => Opcode::SetG(rm),
                    0xB6 => Opcode::MovzxR32Rm8(Register::from(modrm.reg), rm),
                    o => return Err(format!("Not implemented: 0F {:X}", o)),
                }
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
            0x39 => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::CmpRm32R32(rm, Register::from(modrm.reg))
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
            0x6B => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                let value = self.get_code8(0);
                self.inc_eip(1);
                Opcode::IMulR32Rm32Imm8(Register::from(modrm.reg), rm, value)
            }
            0x74 => {
                let addr = self.get_code8(1);
                self.inc_eip(2);
                Opcode::Jz8(addr)
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
                self.inc_eip(1);
                match modrm.reg {
                    0b000 => Opcode::AddRm32Imm32(rm, value as u32),
                    0b101 => Opcode::SubRm32Imm8(rm, value as u8),
                    0b110 => Opcode::XorRm32Imm32(rm, value as u32),
                    0b111 => Opcode::CmpRm32Imm8(rm, value as u8),
                    o => return Err(format!("Not implemented: 83 {:X}", o)),
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
            0x8D => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                if let RM::Memory(addr) = self.calc_rm(&modrm) {
                    Opcode::LeaR32M(Register::from(modrm.reg), addr)
                } else {
                    return Err("expected memory addr".to_string());
                }
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
                    o => return Err(format!("Not implemented: F7 {:X}", o)),
                }
            }
            0xFF => {
                self.inc_eip(1);
                let modrm = self.parse_modrm();
                let rm = self.calc_rm(&modrm);
                Opcode::PushRm32(rm)
            }
            o => return Err(format!("Not implemented: {:X}", o)),
        })
    }
}
