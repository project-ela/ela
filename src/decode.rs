pub mod decoding;

use crate::{
    common::{modrm::ModRM, rex::Rex},
    instruction::{
        mnemonic::Mnemonic,
        operand::{
            memory::{Displacement, Memory},
            register::{self, Register},
            Operand,
        },
        Instruction,
    },
};

pub struct Decoder {
    code: Vec<u8>,
    pos: usize,

    rex: Option<Rex>,
}

pub fn decode(code: &[u8]) -> Vec<Instruction> {
    let mut decoder = Decoder::new(code);
    decoder.decode()
}

impl Decoder {
    pub fn new(code: &[u8]) -> Self {
        Self {
            code: code.to_vec(),
            pos: 0,
            rex: None,
        }
    }

    pub fn decode(&mut self) -> Vec<Instruction> {
        let mut inst = Vec::new();

        while self.pos < self.code.len() {
            inst.push(self.decode_inst());
        }

        inst
    }

    pub fn decode_inst(&mut self) -> Instruction {
        let mut opcode = self.consume_u8();

        if opcode & 0xf0 == 0x40 {
            self.rex = Some(Rex::from_byte(opcode));
            opcode = self.consume_u8();
        } else {
            self.rex = None;
        }

        match opcode {
            0x01 => self.decode_mr(Mnemonic::Add),
            0x03 => self.decode_rm(Mnemonic::Add),
            0x09 => self.decode_mr(Mnemonic::Or),
            0x0b => self.decode_rm(Mnemonic::Or),
            0x0f => {
                let op = self.consume_u8();
                match op {
                    0x84 => self.decode_d32(Mnemonic::Je),
                    0x94 => self.decode_set(Mnemonic::Sete),
                    0x95 => self.decode_set(Mnemonic::Setne),
                    0x9c => self.decode_set(Mnemonic::Setl),
                    0x9d => self.decode_set(Mnemonic::Setge),
                    0x9e => self.decode_set(Mnemonic::Setle),
                    0x9f => self.decode_set(Mnemonic::Setg),
                    0xaf => self.decode_rm(Mnemonic::IMul),
                    _ => panic!(),
                }
            }
            0x21 => self.decode_mr(Mnemonic::And),
            0x23 => self.decode_rm(Mnemonic::And),
            0x29 => self.decode_mr(Mnemonic::Sub),
            0x2b => self.decode_rm(Mnemonic::Sub),
            0x31 => self.decode_mr(Mnemonic::Xor),
            0x33 => self.decode_rm(Mnemonic::Xor),
            0x39 => self.decode_mr(Mnemonic::Cmp),
            0x3b => self.decode_rm(Mnemonic::Cmp),
            0x50..=0x57 => self.decode_o(Mnemonic::Push, opcode - 0x50),
            0x58..=0x5f => self.decode_o(Mnemonic::Pop, opcode - 0x58),
            0x68 => self.decode_i32(Mnemonic::Push),
            0x6a => self.decode_i8(Mnemonic::Push),
            0x74 => self.decode_d8(Mnemonic::Je),
            0x81 => {
                let modrm = ModRM::from_byte(self.consume_u8());
                match modrm.reg {
                    0 => self.decode_mi32(Mnemonic::Add, modrm),
                    1 => self.decode_mi32(Mnemonic::Or, modrm),
                    4 => self.decode_mi32(Mnemonic::And, modrm),
                    5 => self.decode_mi32(Mnemonic::Sub, modrm),
                    6 => self.decode_mi32(Mnemonic::Xor, modrm),
                    7 => self.decode_mi32(Mnemonic::Cmp, modrm),
                    _ => panic!(),
                }
            }
            0x83 => {
                let modrm = ModRM::from_byte(self.consume_u8());
                match modrm.reg {
                    0 => self.decode_mi8(Mnemonic::Add, modrm),
                    1 => self.decode_mi8(Mnemonic::Or, modrm),
                    4 => self.decode_mi8(Mnemonic::And, modrm),
                    5 => self.decode_mi8(Mnemonic::Sub, modrm),
                    6 => self.decode_mi8(Mnemonic::Xor, modrm),
                    7 => self.decode_mi8(Mnemonic::Cmp, modrm),
                    _ => panic!(),
                }
            }
            0x89 => self.decode_mr(Mnemonic::Mov),
            0x8b => self.decode_rm(Mnemonic::Mov),
            0x8f => self.decode_m(Mnemonic::Pop),
            0xc3 => Instruction::new_nullary(Mnemonic::Ret),
            0xc7 => {
                let modrm = ModRM::from_byte(self.consume_u8());
                match modrm.reg {
                    0 => self.decode_mi32(Mnemonic::Mov, modrm),
                    _ => panic!(),
                }
            }
            0xe8 => self.decode_d32(Mnemonic::Call),
            0xe9 => self.decode_d32(Mnemonic::Jmp),
            0xeb => self.decode_d8(Mnemonic::Jmp),
            0xf4 => Instruction::new_nullary(Mnemonic::Hlt),
            0xf7 => {
                let modrm = ModRM::from_byte(self.consume_u8());
                let opr = self.decode_modrm(&modrm);
                match modrm.reg {
                    7 => Instruction::new_unary(Mnemonic::IDiv, opr),
                    _ => panic!(),
                }
            }
            0xff => {
                let modrm = ModRM::from_byte(self.consume_u8());
                let opr = self.decode_modrm(&modrm);
                match modrm.reg {
                    2 => Instruction::new_unary(Mnemonic::Call, opr),
                    4 => Instruction::new_unary(Mnemonic::Jmp, opr),
                    6 => Instruction::new_unary(Mnemonic::Push, opr),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    pub fn pos(&self) -> &usize {
        &self.pos
    }

    fn decode_register_reg(&mut self, num: u8) -> Register {
        let size = if self.rex.as_ref().map_or(false, |rex| rex.w) {
            register::Size::QWord
        } else {
            register::Size::DWord
        };
        let extend = self.rex.as_ref().map_or(false, |rex| rex.r);
        self.decode_register(num, size, extend)
    }

    fn decode_register_rm(&mut self, num: u8) -> Register {
        let size = if self.rex.as_ref().map_or(false, |rex| rex.w) {
            register::Size::QWord
        } else {
            register::Size::DWord
        };
        let extend = self.rex.as_ref().map_or(false, |rex| rex.b);
        self.decode_register(num, size, extend)
    }

    fn decode_register(&mut self, num: u8, size: register::Size, extend: bool) -> Register {
        match size {
            register::Size::QWord => {
                if !extend {
                    match num {
                        0 => Register::Rax,
                        1 => Register::Rcx,
                        2 => Register::Rdx,
                        3 => Register::Rbx,
                        4 => Register::Rsp,
                        5 => Register::Rbp,
                        6 => Register::Rsi,
                        7 => Register::Rdi,
                        _ => panic!(),
                    }
                } else {
                    match num {
                        0 => Register::R8,
                        1 => Register::R9,
                        2 => Register::R10,
                        3 => Register::R11,
                        4 => Register::R12,
                        5 => Register::R13,
                        6 => Register::R14,
                        7 => Register::R15,
                        _ => panic!(),
                    }
                }
            }
            register::Size::DWord => match num {
                0 => Register::Eax,
                1 => Register::Ecx,
                2 => Register::Edx,
                3 => Register::Ebx,
                4 => Register::Esp,
                5 => Register::Ebp,
                6 => Register::Esi,
                7 => Register::Edi,
                _ => panic!(),
            },
            register::Size::Byte => {
                if !extend {
                    match num {
                        0 => Register::Al,
                        2 => Register::Cl,
                        1 => Register::Dl,
                        3 => Register::Bl,
                        _ => panic!(),
                    }
                } else {
                    match num {
                        0 => Register::R8b,
                        1 => Register::R9b,
                        2 => Register::R10b,
                        3 => Register::R11b,
                        4 => Register::R12b,
                        5 => Register::R13b,
                        6 => Register::R14b,
                        7 => Register::R15b,
                        _ => panic!(),
                    }
                }
            }
            _ => panic!(),
        }
    }

    fn decode_modrm(&mut self, modrm: &ModRM) -> Operand {
        match modrm.modval {
            0b00 => Operand::Memory(Memory::new(self.decode_register_rm(modrm.rm), None)),
            0b01 => Operand::Memory(Memory::new(
                self.decode_register_rm(modrm.rm),
                Some(Displacement::Disp8(self.consume_i8())),
            )),
            0b10 => Operand::Memory(Memory::new(
                self.decode_register_rm(modrm.rm),
                Some(Displacement::Disp32(self.consume_i32())),
            )),
            0b11 => Operand::Register(self.decode_register_rm(modrm.rm)),
            _ => panic!(),
        }
    }

    fn consume_u8(&mut self) -> u8 {
        let code = *self.code.get(self.pos).unwrap();
        self.pos += 1;
        code
    }

    fn consume_u32(&mut self) -> u32 {
        let mut ret: u32 = 0;
        for i in 0..4 {
            ret |= (self.consume_u8() as u32) << (i * 8)
        }
        return ret;
    }

    fn consume_i8(&mut self) -> i8 {
        self.consume_u8() as i8
    }

    fn consume_i32(&mut self) -> i32 {
        self.consume_u32() as i32
    }
}
