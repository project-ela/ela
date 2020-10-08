use crate::instruction::{Instruction, Opcode, Operand, Register};
use std::collections::HashMap;

struct Generator {
    output: Vec<u8>,
    labels: HashMap<String, u8>,
}

pub fn generate(insts: Vec<Instruction>) -> Result<Vec<u8>, String> {
    let mut generator = Generator::new();
    generator.generate(insts)
}

impl Generator {
    fn new() -> Self {
        Self {
            output: Vec::new(),
            labels: HashMap::new(),
        }
    }

    fn generate(&mut self, insts: Vec<Instruction>) -> Result<Vec<u8>, String> {
        for inst in insts {
            self.gen_inst(inst)?;
        }
        Ok(self.output.clone())
    }

    fn gen_inst(&mut self, inst: Instruction) -> Result<(), String> {
        match inst {
            Instruction::PseudoOp { .. } => {}
            Instruction::Label { name } => {
                let addr = self.output.len();
                self.labels.insert(name, addr as u8);
            }
            Instruction::NullaryOp(op) => self.gen_nullary_op(op)?,
            Instruction::UnaryOp(op, operand) => self.gen_unary_op(op, operand)?,
            Instruction::BinaryOp(op, operand1, operand2) => {
                self.gen_biary_op(op, operand1, operand2)?
            }
        }
        Ok(())
    }

    fn gen_nullary_op(&mut self, op: Opcode) -> Result<(), String> {
        match op {
            Opcode::Ret => self.gen(0xC3),
            x => return Err(format!("unexpected opcode: {:?}", x)),
        }
        Ok(())
    }

    fn gen_unary_op(&mut self, op: Opcode, operand: Operand) -> Result<(), String> {
        match op {
            Opcode::Push => match operand {
                Operand::Immidiate { value } => {
                    self.gen(0x6A);
                    self.gen(value as u8);
                }
                Operand::Register { reg } => {
                    self.gen(0x50 + reg_to_num(reg));
                }
                Operand::Label { name } => {
                    let addr = self.lookup_label(name)?;
                    self.gen(0x6A);
                    self.gen(addr);
                }
            },
            Opcode::Pop => match operand {
                Operand::Register { reg } => {
                    self.gen(0x58 + reg_to_num(reg));
                }
                x => return Err(format!("unexpected operand: {:?}", x)),
            },
            Opcode::IMul => match operand {
                Operand::Register { reg } => {
                    self.gen(0xF7);
                    self.gen(calc_modrm(0b11, 0b101, reg_to_num(reg)));
                }
                x => return Err(format!("unexpected operand: {:?}", x)),
            },
            Opcode::IDiv => match operand {
                Operand::Register { reg } => {
                    self.gen(0xF7);
                    self.gen(calc_modrm(0b11, 0b111, reg_to_num(reg)));
                }
                x => return Err(format!("unexpected operand: {:?}", x)),
            },
            x => return Err(format!("unexpected opcode: {:?}", x)),
        }
        Ok(())
    }

    fn gen_biary_op(
        &mut self,
        op: Opcode,
        operand1: Operand,
        operand2: Operand,
    ) -> Result<(), String> {
        match op {
            Opcode::Add => {
                let reg1 = match operand1 {
                    Operand::Register { reg } => reg_to_num(reg),
                    x => return Err(format!("unexpected operand: {:?}", x)),
                };
                match operand2 {
                    Operand::Register { reg: reg2 } => {
                        self.gen(0x01);
                        self.gen(calc_modrm(0b11, reg_to_num(reg2), reg1));
                    }
                    Operand::Immidiate { value } => {
                        self.gen(0x83);
                        self.gen(calc_modrm(0b11, 0, reg1));
                        self.gen(value as u8);
                    }
                    Operand::Label { name } => {
                        let addr = self.lookup_label(name)?;
                        self.gen(0x83);
                        self.gen(calc_modrm(0b11, 0, reg1));
                        self.gen(addr as u8);
                    }
                }
            }
            Opcode::Sub => {
                let reg1 = match operand1 {
                    Operand::Register { reg } => reg_to_num(reg),
                    x => return Err(format!("unexpected operand: {:?}", x)),
                };
                match operand2 {
                    Operand::Register { reg: reg2 } => {
                        self.gen(0x29);
                        self.gen(calc_modrm(0b11, reg_to_num(reg2), reg1));
                    }
                    Operand::Immidiate { value } => {
                        self.gen(0x83);
                        self.gen(calc_modrm(0b11, 0b101, reg1));
                        self.gen(value as u8);
                    }
                    Operand::Label { name } => {
                        let addr = self.lookup_label(name)?;
                        self.gen(0x83);
                        self.gen(calc_modrm(0b11, 0b101, reg1));
                        self.gen(addr as u8);
                    }
                }
            }
            Opcode::Xor => {
                let reg1 = match operand1 {
                    Operand::Register { reg } => reg_to_num(reg),
                    x => return Err(format!("unexpected operand: {:?}", x)),
                };
                match operand2 {
                    Operand::Register { reg: reg2 } => {
                        self.gen(0x31);
                        self.gen(calc_modrm(0b11, reg_to_num(reg2), reg1));
                    }
                    Operand::Immidiate { value } => {
                        self.gen(0x83);
                        self.gen(calc_modrm(0b11, 0b110, reg1));
                        self.gen(value as u8);
                    }
                    Operand::Label { name } => {
                        let addr = self.lookup_label(name)?;
                        self.gen(0x83);
                        self.gen(calc_modrm(0b11, 0b110, reg1));
                        self.gen(addr as u8);
                    }
                }
            }
            Opcode::Mov => {
                let reg1 = match operand1 {
                    Operand::Register { reg } => reg_to_num(reg),
                    x => return Err(format!("unexpected operand: {:?}", x)),
                };
                match operand2 {
                    Operand::Register { reg: reg2 } => {
                        self.gen(0x8B);
                        self.gen(calc_modrm(0b11, reg1, reg_to_num(reg2)));
                    }
                    Operand::Immidiate { value } => {
                        self.gen(0xB0 + reg1);
                        self.gen(value as u8);
                    }
                    Operand::Label { name } => {
                        let addr = self.lookup_label(name)?;
                        self.gen(0x80 + reg1);
                        self.gen(addr as u8);
                    }
                }
            }
            x => return Err(format!("unexpected opcode: {:?}", x)),
        }
        Ok(())
    }

    fn lookup_label(&self, name: String) -> Result<u8, String> {
        match self.labels.get(&name) {
            Some(addr) => Ok(*addr),
            None => Err(format!("undefined label: {}", name)),
        }
    }

    fn gen(&mut self, byte: u8) {
        self.output.push(byte);
    }
}

fn reg_to_num(reg: Register) -> u8 {
    match reg {
        Register::Eax => 0,
        Register::Ecx => 1,
        Register::Edx => 2,
        Register::Ebx => 3,
        Register::Esp => 4,
        Register::Ebp => 5,
        Register::Esi => 6,
        Register::Edi => 7,
    }
}

fn calc_modrm(modval: u8, reg: u8, rm: u8) -> u8 {
    modval << 6 | reg << 3 | rm
}
