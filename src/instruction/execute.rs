use crate::emulator::{cpu::Register, Emulator};
use crate::instruction::Opcode;

impl Emulator {
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
