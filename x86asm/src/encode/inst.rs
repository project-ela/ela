use crate::{
    common::{modrm::ModRM, rex::Rex, sib::Sib},
    instruction::operand::{immediate::Immediate, memory::Displacement},
};

#[derive(Default)]
pub struct EncodedInst {
    pub rex: Option<Rex>,
    pub opcode: Vec<u8>,
    pub modrm: Option<ModRM>,
    pub sib: Option<Sib>,
    pub disp: Option<Displacement>,
    pub imm: Option<Immediate>,
}

impl EncodedInst {
    pub fn new(opcode: &[u8]) -> Self {
        let mut enc = Self::default();
        enc.opcode = opcode.to_vec();
        enc
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut enc = Vec::new();

        if let Some(ref rex) = self.rex {
            enc.push(rex.to_byte());
        }

        enc.extend(&self.opcode);

        if let Some(ref modrm) = self.modrm {
            enc.push(modrm.to_byte());
        }

        if let Some(ref sib) = self.sib {
            enc.push(sib.to_byte());
        }

        if let Some(ref disp) = self.disp {
            enc.extend(disp.to_bytes());
        }

        if let Some(ref imm) = self.imm {
            enc.extend(imm.to_bytes());
        }

        enc
    }
}
