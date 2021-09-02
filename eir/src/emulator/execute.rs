use std::cmp::min;

use x86asm::instruction::{
    mnemonic::{self, Mnemonic},
    operand::{
        immediate::Immediate,
        memory::{Displacement, Memory},
        offset::Offset,
        register::{self, Register},
        Operand,
    },
    Instruction,
};

use crate::emulator::{cpu::Flags, Emulator};

use super::value::Value;

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
                let exit_code = self.cpu.get_register8(&Register::Rax);
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

                self.cpu.set_register64(&Register::Rax, buf_len as u64);
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

                self.cpu.set_register64(&Register::Rax, count as u64);
            }
            60 => {
                let exit_code = self.cpu.get_register8(&Register::Rdi);
                println!("Exited with code {}", exit_code);
                std::process::exit(0);
            }
            x => unimplemented!("syscall with {}", x),
        }
    }

    fn exec_unary(&mut self, mnemonic: Mnemonic, opr1: Operand) {
        match mnemonic {
            Mnemonic::Call => {
                let opr1 = self.get_operand64(&opr1);
                self.push64(self.cpu.get_rip()).unwrap();
                self.cpu.set_rip(opr1);
            }
            Mnemonic::IDiv => {
                let lhs = self.cpu.get_register64(&Register::Rax);
                let rhs = self.get_operand64(&opr1);
                let result = self.calc_div(lhs, rhs);
                self.cpu.set_register64(&Register::Rax, result);
                self.cpu.set_register64(&Register::Rdx, lhs % rhs);
            }
            Mnemonic::Je => {
                if self.cpu.get_flag(Flags::ZF) {
                    let opr1 = self.get_operand64(&opr1);
                    self.cpu.set_rip(opr1);
                }
            }
            Mnemonic::Jmp => {
                let opr1 = self.get_operand64(&opr1);
                self.cpu.set_rip(opr1);
            }
            Mnemonic::Push => {
                let opr1 = self.get_operand64(&opr1);
                self.push64(opr1).unwrap();
            }
            Mnemonic::Pop => {
                let value = self.pop64().unwrap();
                self.set_operand(&opr1, Value::Value64(value));
            }
            Mnemonic::Sete
            | Mnemonic::Setg
            | Mnemonic::Setge
            | Mnemonic::Setl
            | Mnemonic::Setle
            | Mnemonic::Setne => self.exec_set(mnemonic, opr1),
            _ => panic!(),
        }
    }

    fn exec_set(&mut self, mnemonic: Mnemonic, opr1: Operand) {
        use Flags::*;
        use Mnemonic::*;
        let flag = match mnemonic {
            Sete => self.cpu.get_flag(ZF),
            Setg => !self.cpu.get_flag(ZF) && self.cpu.get_flag(SF) == self.cpu.get_flag(OF),
            Setge => self.cpu.get_flag(SF) == self.cpu.get_flag(OF),
            Setl => self.cpu.get_flag(SF) != self.cpu.get_flag(OF),
            Setle => self.cpu.get_flag(ZF) || self.cpu.get_flag(SF) != self.cpu.get_flag(OF),
            Setne => !self.cpu.get_flag(ZF),
            _ => panic!(),
        };

        self.set_operand8(&opr1, flag as u8);
    }

    fn exec_binary(&mut self, mnemonic: Mnemonic, opr1: Operand, opr2: Operand) {
        let size = self.check_size(&opr1, &opr2);
        match mnemonic {
            Mnemonic::Add => {
                let lhs = self.get_operand64(&opr1);
                let rhs = self.get_operand64(&opr2);
                let result = self.calc_add(lhs, rhs);
                self.set_operand64(&opr1, result);
            }
            Mnemonic::And => {
                let lhs = self.get_operand64(&opr1);
                let rhs = self.get_operand64(&opr2);
                let result = self.calc_and(lhs, rhs);
                self.set_operand64(&opr1, result);
            }
            Mnemonic::Cmp => {
                let lhs = self.get_operand64(&opr1);
                let rhs = self.get_operand64(&opr2);
                self.calc_sub(lhs, rhs);
            }
            Mnemonic::IMul => {
                let lhs = self.get_operand64(&opr1);
                let rhs = self.get_operand64(&opr2);
                let result = self.calc_mul(lhs, rhs);
                self.set_operand64(&opr1, result);
            }
            Mnemonic::Lea => match opr2 {
                Operand::Memory(mem) => {
                    let addr = self.calc_address(&mem);
                    self.set_operand64(&opr1, addr as u64);
                }
                _ => panic!(),
            },
            Mnemonic::Mov => {
                let value = self.get_operand(&opr2, &size);
                self.set_operand(&opr1, value);
            }
            Mnemonic::Movsx => match opr2 {
                Operand::Memory(mem) => {
                    let addr = self.calc_address(&mem);
                    let value = self.mmu.get_memory8(addr).unwrap();
                    // TODO
                    self.set_operand64(&opr1, value as i8 as i64 as u64);
                }
                _ => panic!(),
            },
            Mnemonic::Or => {
                let lhs = self.get_operand64(&opr1);
                let rhs = self.get_operand64(&opr2);
                let result = self.calc_or(lhs, rhs);
                self.set_operand64(&opr1, result);
            }
            Mnemonic::Sub => {
                let lhs = self.get_operand64(&opr1);
                let rhs = self.get_operand64(&opr2);
                let result = self.calc_sub(lhs, rhs);
                self.set_operand64(&opr1, result);
            }
            Mnemonic::Xor => {
                let lhs = self.get_operand64(&opr1);
                let rhs = self.get_operand64(&opr2);
                let result = self.calc_xor(lhs, rhs);
                self.set_operand64(&opr1, result);
            }
            _ => panic!(),
        }
    }

    fn get_operand64(&self, opr: &Operand) -> u64 {
        self.get_operand(opr, &register::Size::QWord).as_u64()
    }

    fn set_operand64(&mut self, opr: &Operand, value: u64) {
        self.set_operand(opr, Value::Value64(value));
    }

    fn set_operand8(&mut self, opr: &Operand, value: u8) {
        self.set_operand(opr, Value::Value8(value));
    }

    fn get_operand(&self, opr: &Operand, size: &register::Size) -> Value {
        match opr {
            Operand::Immediate(imm) => match imm {
                Immediate::Imm8(value) => Value::Value8(*value as u8),
                Immediate::Imm32(value) => Value::Value64(*value as u64),
            },
            Operand::Register(reg) => self.cpu.get_register(reg),
            Operand::Memory(mem) => {
                let addr = self.calc_address(mem);
                self.mmu.get_memory(addr, size).unwrap()
            }
            Operand::Offset(off) => {
                let rip = self.cpu.get_rip() as i64;
                let off = match off {
                    Offset::Off8(off) => *off as i64,
                    Offset::Off32(off) => *off as i64,
                };
                Value::Value64((rip + off) as u64)
            }
        }
    }

    fn set_operand(&mut self, opr: &Operand, value: Value) {
        match opr {
            Operand::Register(reg) => self.cpu.set_register(reg, value),
            Operand::Memory(mem) => {
                let addr = self.calc_address(mem);
                self.mmu.set_memory(addr, value).unwrap();
            }
            _ => panic!(),
        }
    }

    fn check_size(&self, opr1: &Operand, opr2: &Operand) -> register::Size {
        match (opr1, opr2) {
            (Operand::Register(reg1), Operand::Register(reg2)) => {
                if reg1.size() != reg2.size() {
                    panic!("operand type mismatch");
                }
                reg1.size()
            }
            (Operand::Register(reg), Operand::Memory(_))
            | (Operand::Memory(_), Operand::Register(reg))
            | (Operand::Register(reg), Operand::Immediate(_)) => reg.size(),
            (Operand::Memory(_), Operand::Immediate(imm)) => match imm {
                Immediate::Imm8(_) => register::Size::Byte,
                Immediate::Imm32(_) => register::Size::QWord,
            },
            _ => register::Size::QWord,
        }
    }

    fn calc_address(&self, mem: &Memory) -> usize {
        let base = if let Some(ref reg) = mem.base {
            match reg {
                Register::Rip => self.cpu.get_rip(),
                x => self.cpu.get_register64(x),
            }
        } else {
            0
        } as isize;

        let disp = mem.disp.as_ref().map_or(0, |disp| match disp {
            Displacement::Disp8(value) => *value as isize,
            Displacement::Disp32(value) => *value as isize,
        });

        (base + disp) as usize
    }
}
