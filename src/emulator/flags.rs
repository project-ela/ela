use super::{cpu::Flags, Emulator};

impl Emulator {
    pub fn update_eflags_add(&mut self, lhs: u32, rhs: u32, result: u64) {
        self.update_eflags_sub(lhs, rhs, result);
    }

    pub fn update_eflags_sub(&mut self, lhs: u32, rhs: u32, result: u64) {
        let lhs_sign = lhs >> 31;
        let rhs_sign = rhs >> 31;
        let result_sign = (result >> 31) as u32;
        self.cpu.set_flag(Flags::CF, (result >> 32) != 0);
        self.cpu.set_flag(Flags::ZF, result == 0);
        self.cpu.set_flag(Flags::SF, result_sign != 0);
        self.cpu.set_flag(
            Flags::OF,
            (lhs_sign != rhs_sign) && (lhs_sign != result_sign),
        );
    }

    pub fn update_eflags_xor(&mut self, result: u64) {
        let result_sign = (result >> 31) as u32;
        self.cpu.set_flag(Flags::CF, false);
        self.cpu.set_flag(Flags::ZF, result != 0);
        self.cpu.set_flag(Flags::SF, result_sign != 0);
        self.cpu.set_flag(Flags::OF, false);
    }
}
