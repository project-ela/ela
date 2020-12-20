use x86asm::instruction::{
    mnemonic::{self, Mnemonic},
    operand::{
        immediate::Immediate,
        memory::{Displacement, Memory},
        offset::Offset,
        register::Register,
        Operand,
    },
    Instruction,
};

use crate::emulator::{cpu::Flags, Emulator};

impl Emulator {
    pub fn exec(&mut self, inst: Instruction) {
        let typ = inst.mnenomic.typ();
        match typ {
            mnemonic::Type::Nullary => self.exec_nullary(inst.mnenomic),
            mnemonic::Type::Unary => {
                let opr1 = inst.operand1.unwrap();
                self.exec_unary(inst.mnenomic, opr1);
            }
            mnemonic::Type::Binary => {
                let opr1 = inst.operand1.unwrap();
                let opr2 = inst.operand2.unwrap();
                self.exec_binary(inst.mnenomic, opr1, opr2);
            }
        }
    }

    fn exec_nullary(&mut self, mnemonic: Mnemonic) {
        match mnemonic {
            Mnemonic::Ret => {
                let new_rip = self.pop64();
                self.cpu.set_rip(new_rip);
            }
            _ => panic!(),
        }
    }

    fn exec_unary(&mut self, mnemonic: Mnemonic, opr1: Operand) {
        match mnemonic {
            Mnemonic::Call => {
                let opr1 = self.get_operand(&opr1);
                self.push64(self.cpu.get_rip());
                self.cpu.set_rip(opr1);
            }
            Mnemonic::IDiv => {
                let lhs = self.cpu.get_register(&Register::Rax);
                let rhs = self.get_operand(&opr1);
                let result = self.calc_div(lhs, rhs);
                self.cpu.set_register(&Register::Rax, result);
            }
            Mnemonic::Je => {
                if self.cpu.get_flag(Flags::ZF) {
                    let opr1 = self.get_operand(&opr1);
                    self.cpu.set_rip(opr1);
                }
            }
            Mnemonic::Jmp => {
                let opr1 = self.get_operand(&opr1);
                self.cpu.set_rip(opr1);
            }
            Mnemonic::Push => {
                let opr1 = self.get_operand(&opr1);
                self.push64(opr1);
            }
            Mnemonic::Pop => {
                let value = self.pop64();
                self.set_operand(&opr1, value);
            }
            Mnemonic::Sete => self.set_operand(&opr1, self.cpu.get_flag(Flags::ZF) as u64),
            Mnemonic::Setg => self.set_operand(
                &opr1,
                (!self.cpu.get_flag(Flags::ZF)
                    && self.cpu.get_flag(Flags::SF) == self.cpu.get_flag(Flags::OF))
                    as u64,
            ),
            Mnemonic::Setge => self.set_operand(
                &opr1,
                (self.cpu.get_flag(Flags::SF) == self.cpu.get_flag(Flags::OF)) as u64,
            ),
            Mnemonic::Setl => self.set_operand(
                &opr1,
                (self.cpu.get_flag(Flags::SF) != self.cpu.get_flag(Flags::OF)) as u64,
            ),
            Mnemonic::Setle => self.set_operand(
                &opr1,
                (self.cpu.get_flag(Flags::ZF)
                    || self.cpu.get_flag(Flags::SF) != self.cpu.get_flag(Flags::OF))
                    as u64,
            ),
            Mnemonic::Setne => self.set_operand(&opr1, !self.cpu.get_flag(Flags::ZF) as u64),
            _ => panic!(),
        }
    }

    fn exec_binary(&mut self, mnemonic: Mnemonic, opr1: Operand, opr2: Operand) {
        match mnemonic {
            Mnemonic::Add => {
                let lhs = self.get_operand(&opr1);
                let rhs = self.get_operand(&opr2);
                let result = self.calc_add(lhs, rhs);
                self.set_operand(&opr1, result);
            }
            Mnemonic::And => {
                let lhs = self.get_operand(&opr1);
                let rhs = self.get_operand(&opr2);
                let result = self.calc_and(lhs, rhs);
                self.set_operand(&opr1, result);
            }
            Mnemonic::Cmp => {
                let lhs = self.get_operand(&opr1);
                let rhs = self.get_operand(&opr2);
                self.calc_sub(lhs, rhs);
            }
            Mnemonic::IMul => {
                let lhs = self.get_operand(&opr1);
                let rhs = self.get_operand(&opr2);
                let result = self.calc_mul(lhs, rhs);
                self.set_operand(&opr1, result);
            }
            Mnemonic::Mov => {
                let value = self.get_operand(&opr2);
                self.set_operand(&opr1, value);
            }
            Mnemonic::Or => {
                let lhs = self.get_operand(&opr1);
                let rhs = self.get_operand(&opr2);
                let result = self.calc_or(lhs, rhs);
                self.set_operand(&opr1, result);
            }
            Mnemonic::Sub => {
                let lhs = self.get_operand(&opr1);
                let rhs = self.get_operand(&opr2);
                let result = self.calc_sub(lhs, rhs);
                self.set_operand(&opr1, result);
            }
            Mnemonic::Xor => {
                let lhs = self.get_operand(&opr1);
                let rhs = self.get_operand(&opr2);
                let result = self.calc_xor(lhs, rhs);
                self.set_operand(&opr1, result);
            }
            _ => panic!(),
        }
    }

    fn get_operand(&self, opr: &Operand) -> u64 {
        match opr {
            Operand::Immediate(imm) => match imm {
                Immediate::Imm8(value) => *value as u64,
                Immediate::Imm32(value) => *value as u64,
            },
            Operand::Register(reg) => self.cpu.get_register(reg),
            Operand::Memory(mem) => self.mmu.get_memory32(self.calc_address(mem)) as u64,
            Operand::Offset(off) => match off {
                Offset::Off8(value) => (self.cpu.get_rip() as i64 + *value as i64) as u64,
                Offset::Off32(value) => (self.cpu.get_rip() as i64 + *value as i64) as u64,
            },
        }
    }

    fn set_operand(&mut self, opr: &Operand, value: u64) {
        match opr {
            Operand::Register(reg) => self.cpu.set_register(reg, value),
            Operand::Memory(mem) => self.mmu.set_memory32(self.calc_address(mem), value as u32),
            _ => panic!(),
        }
    }

    fn calc_address(&self, mem: &Memory) -> usize {
        let base = self.cpu.get_register(&mem.base) as isize;
        let disp = mem.disp.as_ref().map_or(0, |disp| match disp {
            Displacement::Disp8(value) => *value as isize,
            Displacement::Disp32(value) => *value as isize,
        });

        (base + disp) as usize
    }
}
