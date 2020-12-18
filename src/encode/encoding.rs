use crate::{
    common::{modrm::ModRM, rex::Rex},
    instruction::operand::{
        immediate::Immediate,
        memory::{Displacement, Memory},
        offset::Offset,
        register::{self, Register},
    },
};

use super::inst::EncodedInst;

pub enum RM<'a> {
    Register(&'a Register),
    Memory(&'a Memory),
}

pub fn encode_m(opcode: &[u8], opr1: RM) -> EncodedInst {
    let mut enc = EncodedInst::new(opcode);
    enc.rex = encode_rex(Some(&opr1), None);
    enc.modrm = Some(encode_modrm(&opr1));
    enc.disp = encode_disp(&opr1);
    enc
}

pub fn encode_o(opcode: u8, opr1: &Register) -> EncodedInst {
    let mut enc = EncodedInst::new(&[opcode + opr1.number()]);
    if opr1.only_in_64bit() {
        enc.rex = Some(Rex::new(false, false, false, opr1.only_in_64bit()));
    }
    enc
}

pub fn encode_i(opcode: &[u8], opr1: &Immediate) -> EncodedInst {
    let mut enc = EncodedInst::new(opcode);
    enc.imm = Some(opr1.clone());
    enc
}

pub fn encode_d(opcode: &[u8], opr1: &Offset) -> EncodedInst {
    let mut enc = EncodedInst::new(opcode);
    enc.imm = match opr1 {
        Offset::Off8(value) => Some(Immediate::Imm8(*value)),
        Offset::Off32(value) => Some(Immediate::Imm32(*value)),
    };
    enc
}

pub fn encode_mi(opcode: &[u8], opr1: RM, opr2: &Immediate) -> EncodedInst {
    let mut enc = EncodedInst::new(opcode);
    enc.rex = encode_rex(Some(&opr1), None);
    enc.modrm = Some(encode_modrm(&opr1));
    enc.disp = encode_disp(&opr1);
    enc.imm = Some(opr2.clone());
    enc
}

pub fn encode_mr(opcode: &[u8], opr1: RM, opr2: &Register) -> EncodedInst {
    let mut enc = EncodedInst::new(opcode);
    enc.rex = encode_rex(Some(&opr1), Some(opr2));
    enc.modrm = Some({
        let mut modrm = encode_modrm(&opr1);
        modrm.reg = opr2.number();
        modrm
    });
    enc.disp = encode_disp(&opr1);
    enc
}

pub fn encode_rm(opcode: &[u8], opr1: &Register, opr2: RM) -> EncodedInst {
    let mut enc = EncodedInst::new(opcode);
    enc.rex = encode_rex(Some(&opr2), Some(opr1));
    enc.modrm = Some({
        let mut modrm = encode_modrm(&opr2);
        modrm.reg = opr1.number();
        modrm
    });
    enc.disp = encode_disp(&opr2);
    enc
}

pub fn encode_set(opcode: &[u8], opr1: RM) -> EncodedInst {
    let mut enc = EncodedInst::new(opcode);
    enc.rex = match opr1 {
        RM::Register(reg) => {
            if reg.only_in_64bit() {
                Some(Rex::new(false, false, false, reg.only_in_64bit()))
            } else {
                None
            }
        }
        RM::Memory(_) => encode_rex(Some(&opr1), None),
    };
    enc.modrm = Some(encode_modrm(&opr1));
    enc.disp = encode_disp(&opr1);
    enc
}

// TODO fix REX.W is always true
// e.g. push r8 => expected: [0x41, 0x58], actual: [0x49, 0x58]
fn encode_rex(rm: Option<&RM>, reg: Option<&Register>) -> Option<Rex> {
    let reg_rm = match rm {
        Some(RM::Register(reg)) => reg,
        Some(RM::Memory(mem)) => &mem.base,
        None => &Register::Al,
    };
    let reg_reg = reg.unwrap_or(&Register::Al);

    if reg_rm.size() != register::Size::QWord
        && reg_reg.size() != register::Size::QWord
        && !reg_rm.only_in_64bit()
        && !reg_reg.only_in_64bit()
    {
        return None;
    }

    Some(Rex::new(
        true,
        reg_reg.only_in_64bit(),
        false,
        reg_rm.only_in_64bit(),
    ))
}

fn encode_modrm(rm: &RM) -> ModRM {
    match rm {
        RM::Memory(mem) => match &mem.disp {
            None => ModRM::new(0b00, 0, mem.base.number()),
            Some(Displacement::Disp8(_)) => ModRM::new(0b01, 0, mem.base.number()),
            Some(Displacement::Disp32(_)) => ModRM::new(0b10, 0, mem.base.number()),
        },
        RM::Register(reg) => ModRM::new(0b11, 0, reg.number()),
    }
}

fn encode_disp(rm: &RM) -> Option<Displacement> {
    match rm {
        RM::Memory(Memory { disp, .. }) => disp.clone(),
        _ => None,
    }
}
