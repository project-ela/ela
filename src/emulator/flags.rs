use super::{cpu::Flags, Emulator};

// TODO support 32/16/8bit value
impl Emulator {
    pub fn calc_add(&mut self, lhs: u64, rhs: u64) -> u64 {
        let (result, result_overflow) = (lhs as i64).overflowing_add(rhs as i64);
        let result_carry = (lhs as u128).wrapping_add(rhs as u128) >> 64;
        let result_sign = result >> 63;
        self.cpu.set_flag(Flags::CF, result_carry != 0);
        self.cpu.set_flag(Flags::PF, check_parity(result as u8));
        self.cpu.set_flag(Flags::ZF, result == 0);
        self.cpu.set_flag(Flags::SF, result_sign != 0);
        self.cpu.set_flag(Flags::OF, result_overflow);

        result as u64
    }

    pub fn calc_sub(&mut self, lhs: u64, rhs: u64) -> u64 {
        let (result, result_overflow) = (lhs as i64).overflowing_sub(rhs as i64);
        let result_carry = (lhs as u128).wrapping_sub(rhs as u128) >> 64;
        let result_sign = result >> 63;
        self.cpu.set_flag(Flags::CF, result_carry != 0);
        self.cpu.set_flag(Flags::PF, check_parity(result as u8));
        self.cpu.set_flag(Flags::ZF, result == 0);
        self.cpu.set_flag(Flags::SF, result_sign != 0);
        self.cpu.set_flag(Flags::OF, result_overflow);

        result as u64
    }

    pub fn calc_mul(&mut self, lhs: u64, rhs: u64) -> u64 {
        let (result, result_overflow) = (lhs as i64).overflowing_mul(rhs as i64);
        let result_carry = (lhs as u128).wrapping_mul(rhs as u128) >> 64;
        let result_sign = result >> 63;
        self.cpu.set_flag(Flags::CF, result_carry != 0);
        self.cpu.set_flag(Flags::PF, check_parity(result as u8));
        self.cpu.set_flag(Flags::ZF, result == 0);
        self.cpu.set_flag(Flags::SF, result_sign != 0);
        self.cpu.set_flag(Flags::OF, result_overflow);

        result as u64
    }

    pub fn calc_div(&mut self, lhs: u64, rhs: u64) -> u64 {
        let (result, result_overflow) = (lhs as i64).overflowing_div(rhs as i64);
        let result_carry = (lhs as u128).wrapping_div(rhs as u128) >> 64;
        let result_sign = result >> 63;
        self.cpu.set_flag(Flags::CF, result_carry != 0);
        self.cpu.set_flag(Flags::PF, check_parity(result as u8));
        self.cpu.set_flag(Flags::ZF, result == 0);
        self.cpu.set_flag(Flags::SF, result_sign != 0);
        self.cpu.set_flag(Flags::OF, result_overflow);

        result as u64
    }

    pub fn calc_and(&mut self, lhs: u64, rhs: u64) -> u64 {
        let result = lhs & rhs;
        let result_sign = result >> 63;
        self.cpu.set_flag(Flags::CF, false);
        self.cpu.set_flag(Flags::PF, check_parity(result as u8));
        self.cpu.set_flag(Flags::ZF, result == 0);
        self.cpu.set_flag(Flags::SF, result_sign != 0);
        self.cpu.set_flag(Flags::OF, false);

        result
    }

    pub fn calc_or(&mut self, lhs: u64, rhs: u64) -> u64 {
        let result = lhs | rhs;
        let result_sign = result >> 63;
        self.cpu.set_flag(Flags::CF, false);
        self.cpu.set_flag(Flags::PF, check_parity(result as u8));
        self.cpu.set_flag(Flags::ZF, result == 0);
        self.cpu.set_flag(Flags::SF, result_sign != 0);
        self.cpu.set_flag(Flags::OF, false);

        result
    }

    pub fn calc_xor(&mut self, lhs: u64, rhs: u64) -> u64 {
        let result = lhs ^ rhs;
        let result_sign = result >> 63;
        self.cpu.set_flag(Flags::CF, false);
        self.cpu.set_flag(Flags::PF, check_parity(result as u8));
        self.cpu.set_flag(Flags::ZF, result == 0);
        self.cpu.set_flag(Flags::SF, result_sign != 0);
        self.cpu.set_flag(Flags::OF, false);

        result
    }
}

/// 1になっているビットが偶数個の場合にtrueを返す
fn check_parity(value: u8) -> bool {
    value.count_ones() % 2 == 0
}
