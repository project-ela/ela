use crate::{
    emulator::{
        cpu::{Register, EFLAGS},
        Emulator,
    },
    instruction::Opcode,
};

impl Emulator {
    pub fn exec(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::AddRm8R8(rm, reg) => {
                self.set_rm(rm, self.get_register(reg));
            }
            Opcode::AddRm32R32(rm, reg) => {
                let lhs = self.get_rm(rm);
                let rhs = self.get_register(reg);
                let result = (lhs as u64).wrapping_add(rhs as u64);
                self.update_eflags_add(lhs, rhs, result);
                self.set_rm(rm, result as u32);
            }
            Opcode::Jz32(diff) => {
                if self.get_eflag(EFLAGS::ZF) {
                    self.inc_eip(diff);
                }
            }
            Opcode::SetE(rm) => self.set_rm(rm, self.get_eflag(EFLAGS::ZF) as u32),
            Opcode::SetNE(rm) => self.set_rm(rm, !self.get_eflag(EFLAGS::ZF) as u32),
            Opcode::SetL(rm) => self.set_rm(
                rm,
                (self.get_eflag(EFLAGS::SF) != self.get_eflag(EFLAGS::OF)) as u32,
            ),
            Opcode::SetGE(rm) => self.set_rm(
                rm,
                (self.get_eflag(EFLAGS::SF) == self.get_eflag(EFLAGS::OF)) as u32,
            ),
            Opcode::SetLE(rm) => self.set_rm(
                rm,
                (self.get_eflag(EFLAGS::ZF)
                    || self.get_eflag(EFLAGS::SF) != self.get_eflag(EFLAGS::OF))
                    as u32,
            ),
            Opcode::SetG(rm) => self.set_rm(
                rm,
                (!self.get_eflag(EFLAGS::ZF)
                    && self.get_eflag(EFLAGS::SF) == self.get_eflag(EFLAGS::OF))
                    as u32,
            ),
            Opcode::MovzxR32Rm8(reg, rm) => self.set_register(reg, self.get_rm(rm)),
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
            Opcode::CmpRm32R32(rm, reg) => {
                let lhs = self.get_rm(rm);
                let rhs = self.get_register(reg);
                let result = (lhs as u64).wrapping_sub(rhs as u64);
                self.update_eflags_sub(lhs, rhs, result)
            }
            Opcode::PushR32(reg) => self.push32(self.get_register(reg)),
            Opcode::PopR32(reg) => {
                let value = self.pop32();
                self.set_register(reg, value);
            }
            Opcode::IMulR32Rm32Imm8(reg, rm, value) => {
                let lhs = self.get_rm(rm);
                let result = lhs.wrapping_mul(value as u32);
                //TODO: update eflags
                self.set_register(Register::from(reg), result);
            }
            Opcode::Jz8(addr) => {
                if self.get_eflag(EFLAGS::ZF) {
                    self.set_register(Register::EIP, addr as u32);
                }
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
            Opcode::SubRm32Imm8(rm, value) => {
                let lhs = self.get_rm(rm);
                let result = (lhs as u64).wrapping_sub(value as u64);
                self.update_eflags_sub(lhs, value as u32, result);
                self.set_rm(rm, result as u32);
            }
            Opcode::XorRm32Imm32(rm, value) => {
                let lhs = self.get_rm(rm);
                let result = lhs ^ value;
                self.update_eflags_xor(result as u64);
                self.set_rm(rm, result);
            }
            Opcode::CmpRm32Imm8(rm, value) => {
                let lhs = self.get_rm(rm);
                let result = (lhs as u64).wrapping_sub(value as u64);
                self.update_eflags_sub(lhs, value as u32, result)
            }
            Opcode::MovRm32R32(rm, reg) => self.set_rm(rm, self.get_register(reg)),
            Opcode::MovR32Rm32(reg, rm) => self.set_register(reg, self.get_rm(rm)),
            Opcode::LeaR32M(reg, addr) => {
                self.set_register(reg, addr as u32);
            }
            Opcode::PopRm32(rm) => {
                let value = self.pop32();
                self.set_rm(rm, value);
            }
            Opcode::Nop => {}
            Opcode::MovR32Imm32(reg, value) => self.set_register(reg, value),
            Opcode::Ret => {
                let value = self.get_register(Register::EAX);
                println!("Exited with {}", value);
                std::process::exit(0);
            }
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
}
