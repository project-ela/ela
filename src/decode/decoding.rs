use crate::{
    common::modrm::ModRM,
    instruction::{
        mnemonic::Mnemonic,
        operand::{immediate::Immediate, offset::Offset, register, Operand},
        Instruction,
    },
};

use super::Decoder;

impl Decoder {
    pub fn decode_m(&mut self, mnemonic: Mnemonic) -> Instruction {
        let modrm = ModRM::from_byte(self.consume_u8());
        let opr = self.decode_modrm(&modrm);
        Instruction::new_unary(mnemonic, opr)
    }

    pub fn decode_o(&mut self, mnemonic: Mnemonic, reg: u8) -> Instruction {
        let extend = self.rex.as_ref().map_or(false, |rex| rex.b);
        let reg = self.decode_register(reg, register::Size::QWord, extend);
        Instruction::new_unary(mnemonic, Operand::Register(reg))
    }

    pub fn decode_i8(&mut self, mnemonic: Mnemonic) -> Instruction {
        let imm = Immediate::Imm8(self.consume_i8());
        Instruction::new_unary(mnemonic, Operand::Immediate(imm))
    }

    pub fn decode_i32(&mut self, mnemonic: Mnemonic) -> Instruction {
        let imm = Immediate::Imm32(self.consume_i32());
        Instruction::new_unary(mnemonic, Operand::Immediate(imm))
    }

    pub fn decode_d8(&mut self, mnemonic: Mnemonic) -> Instruction {
        let off = Offset::Off8(self.consume_i8());
        Instruction::new_unary(mnemonic, Operand::Offset(off))
    }

    pub fn decode_d32(&mut self, mnemonic: Mnemonic) -> Instruction {
        let off = Offset::Off32(self.consume_i32());
        Instruction::new_unary(mnemonic, Operand::Offset(off))
    }

    pub fn decode_mi8(&mut self, mnemonic: Mnemonic, modrm: ModRM) -> Instruction {
        let opr1 = self.decode_modrm(&modrm);
        let opr2 = Immediate::Imm8(self.consume_i8());
        Instruction::new_binary(mnemonic, opr1, Operand::Immediate(opr2))
    }

    pub fn decode_mi32(&mut self, mnemonic: Mnemonic, modrm: ModRM) -> Instruction {
        let opr1 = self.decode_modrm(&modrm);
        let opr2 = Immediate::Imm32(self.consume_i32());
        Instruction::new_binary(mnemonic, opr1, Operand::Immediate(opr2))
    }

    pub fn decode_mr(&mut self, mnemonic: Mnemonic) -> Instruction {
        let modrm = ModRM::from_byte(self.consume_u8());
        let opr1 = self.decode_modrm(&modrm);
        let opr2 = self.decode_register_reg(modrm.reg);
        Instruction::new_binary(mnemonic, opr1, Operand::Register(opr2))
    }

    pub fn decode_rm(&mut self, mnemonic: Mnemonic) -> Instruction {
        let modrm = ModRM::from_byte(self.consume_u8());
        let opr1 = self.decode_register_reg(modrm.reg);
        let opr2 = self.decode_modrm(&modrm);
        Instruction::new_binary(mnemonic, Operand::Register(opr1), opr2)
    }

    // TODO
    pub fn decode_rmi8(&mut self, mnemonic: Mnemonic) -> Instruction {
        let modrm = ModRM::from_byte(self.consume_u8());
        let opr1 = self.decode_register_reg(modrm.reg);
        let opr2 = Immediate::Imm8(self.consume_i8());
        Instruction::new_binary(mnemonic, Operand::Register(opr1), Operand::Immediate(opr2))
    }

    // TODO
    pub fn decode_rmi32(&mut self, mnemonic: Mnemonic) -> Instruction {
        let modrm = ModRM::from_byte(self.consume_u8());
        let opr1 = self.decode_register_reg(modrm.reg);
        let opr2 = Immediate::Imm32(self.consume_i32());
        Instruction::new_binary(mnemonic, Operand::Register(opr1), Operand::Immediate(opr2))
    }

    pub fn decode_set(&mut self, mnemonic: Mnemonic) -> Instruction {
        let modrm = ModRM::from_byte(self.consume_u8());
        let extend = self.rex.as_ref().map_or(false, |rex| rex.b);
        let opr = self.decode_register(modrm.rm, register::Size::Byte, extend);
        Instruction::new_unary(mnemonic, Operand::Register(opr))
    }
}
