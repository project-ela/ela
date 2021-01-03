use std::cmp::min;

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
            Mnemonic::Hlt => {
                let exit_code = self.cpu.get_register(&Register::Rax) as u8;
                println!("Exited with code {}", exit_code);
                std::process::exit(0);
            }
            Mnemonic::Ret => {
                let new_rip = self.pop64().unwrap();
                self.cpu.set_rip(new_rip);
            }
            Mnemonic::Syscall => self.exec_syscall(),
            _ => panic!(),
        }
    }

    fn exec_syscall(&mut self) {
        let rax = self.cpu.get_register64(&Register::Rax);
        match rax {
            0 => {
                let fd = self.cpu.get_register64(&Register::Rdi);
                let buf_addr = self.cpu.get_register64(&Register::Rsi) as usize;
                let count = self.cpu.get_register64(&Register::Rdx) as usize;

                if fd != 0 {
                    unimplemented!();
                }
                let mut buf = String::new();
                let mut buf_len = std::io::stdin().read_line(&mut buf).unwrap();
                buf_len = min(buf_len, count);

                let buf_bytes = buf.as_bytes();
                for i in 0..buf_len {
                    let addr = buf_addr + i;
                    let value = buf_bytes[i];
                    self.mmu.set_memory8(addr, value).unwrap();
                }
            }
            1 => {
                let fd = self.cpu.get_register64(&Register::Rdi);
                let buf_addr = self.cpu.get_register64(&Register::Rsi) as usize;
                let count = self.cpu.get_register64(&Register::Rdx) as usize;

                let mut buf = String::new();
                for i in 0..count {
                    let addr = buf_addr + i;
                    let value = self.mmu.get_memory8(addr).unwrap();
                    buf.push(value as char);
                }

                match fd {
                    1 => print!("{}", buf),
                    2 => eprint!("{}", buf),
                    _ => unimplemented!(),
                }
            }
            60 => {
                let exit_code = self.cpu.get_register(&Register::Rdi) as u8;
                println!("Exited with code {}", exit_code);
                std::process::exit(0);
            }
            x => unimplemented!("syscall with {}", x),
        }
    }

    fn exec_unary(&mut self, mnemonic: Mnemonic, opr1: Operand) {
        match mnemonic {
            Mnemonic::Call => {
                let opr1 = self.get_operand(&opr1);
                self.push64(self.cpu.get_rip()).unwrap();
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
                self.push64(opr1).unwrap();
            }
            Mnemonic::Pop => {
                let value = self.pop64().unwrap();
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
            Mnemonic::Lea => match opr2 {
                Operand::Memory(mem) => {
                    let addr = self.calc_address(&mem);
                    self.set_operand(&opr1, addr as u64);
                }
                _ => panic!(),
            },
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
            Operand::Memory(mem) => self.mmu.get_memory64(self.calc_address(mem)).unwrap(),
            Operand::Offset(off) => match off {
                Offset::Off8(value) => (self.cpu.get_rip() as i64 + *value as i64) as u64,
                Offset::Off32(value) => (self.cpu.get_rip() as i64 + *value as i64) as u64,
            },
        }
    }

    fn set_operand(&mut self, opr: &Operand, value: u64) {
        match opr {
            Operand::Register(reg) => self.cpu.set_register(reg, value),
            Operand::Memory(mem) => self
                .mmu
                .set_memory64(self.calc_address(mem), value)
                .unwrap(),
            _ => panic!(),
        }
    }

    fn calc_address(&self, mem: &Memory) -> usize {
        let base = mem
            .base
            .as_ref()
            .map_or(0, |base| self.cpu.get_register64(&base) as isize);
        let disp = mem.disp.as_ref().map_or(0, |disp| match disp {
            Displacement::Disp8(value) => *value as isize,
            Displacement::Disp32(value) => *value as isize,
        });

        (base + disp) as usize
    }
}
