pub mod encoding;
pub mod inst;

use encoding::RM;
use inst::EncodedInst;
use register::Register;

use crate::instruction::{
    mnemonic::{self, Mnemonic},
    operand::{immediate::Immediate, offset::Offset, register, Operand},
    Instruction,
};

pub fn encode(inst: &Instruction) -> Vec<u8> {
    let typ = inst.mnenomic.typ();
    let enc = match typ {
        mnemonic::Type::Nullary => encode_nullary_op(inst),
        mnemonic::Type::Unary => encode_unary_op(inst),
        mnemonic::Type::Binary => encode_binary_op(inst),
    };

    enc.to_bytes()
}

fn encode_nullary_op(inst: &Instruction) -> EncodedInst {
    if inst.operand1.is_some() || inst.operand2.is_some() {
        panic!("number of operands mismatched");
    }

    match inst.mnenomic {
        Mnemonic::Hlt => EncodedInst::new(&[0xf4]),
        Mnemonic::Ret => EncodedInst::new(&[0xc3]),
        Mnemonic::Syscall => EncodedInst::new(&[0x0f, 0x05]),
        _ => panic!(),
    }
}

fn encode_unary_op(inst: &Instruction) -> EncodedInst {
    let opr1 = inst.operand1.as_ref().expect("first operand is required");
    if inst.operand2.is_some() {
        panic!("number of operands mismatched");
    }

    match inst.mnenomic {
        Mnemonic::Call => match opr1 {
            Operand::Offset(off) => match off {
                Offset::Off8(_) => panic!(),
                Offset::Off32(_) => encoding::encode_d(&[0xe8], off),
            },
            Operand::Register(_) | Operand::Memory(_) => {
                encoding::encode_m(&[0xff], opr1.to_rm()).set_reg(2)
            }
            _ => panic!(),
        },
        Mnemonic::IDiv => match opr1 {
            Operand::Register(_) | Operand::Memory(_) => {
                encoding::encode_m(&[0xf7], opr1.to_rm()).set_reg(7)
            }
            _ => panic!(),
        },
        Mnemonic::Pop => match opr1 {
            Operand::Register(reg) => {
                encoding::encode_o(0x58, reg.expect_size(register::Size::QWord))
            }
            Operand::Memory(mem) => encoding::encode_m(&[0x8f], RM::Memory(mem)),
            _ => panic!(),
        },
        Mnemonic::Push => match opr1 {
            Operand::Immediate(imm) => match imm {
                Immediate::Imm8(_) => encoding::encode_i(&[0x6a], imm),
                Immediate::Imm32(_) => encoding::encode_i(&[0x68], imm),
            },
            Operand::Register(reg) => {
                encoding::encode_o(0x50, reg.expect_size(register::Size::QWord))
            }
            Operand::Memory(mem) => encoding::encode_m(&[0xff], RM::Memory(mem)).set_reg(6),
            _ => panic!(),
        },
        Mnemonic::Je => match opr1 {
            Operand::Offset(off) => match off {
                Offset::Off8(_) => encoding::encode_d(&[0x74], off),
                Offset::Off32(_) => encoding::encode_d(&[0x0f, 0x84], off),
            },
            _ => panic!(),
        },
        Mnemonic::Jmp => match opr1 {
            Operand::Offset(off) => match off {
                Offset::Off8(_) => encoding::encode_d(&[0xeb], off),
                Offset::Off32(_) => encoding::encode_d(&[0xe9], off),
            },
            Operand::Register(_) | Operand::Memory(_) => {
                encoding::encode_m(&[0xff], opr1.to_rm()).set_reg(4)
            }
            _ => panic!(),
        },
        Mnemonic::Sete => match opr1 {
            Operand::Register(_) | Operand::Memory(_) => {
                encoding::encode_set(&[0x0f, 0x94], opr1.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Setg => match opr1 {
            Operand::Register(_) | Operand::Memory(_) => {
                encoding::encode_set(&[0x0f, 0x9f], opr1.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Setge => match opr1 {
            Operand::Register(_) | Operand::Memory(_) => {
                encoding::encode_set(&[0x0f, 0x9d], opr1.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Setl => match opr1 {
            Operand::Register(_) | Operand::Memory(_) => {
                encoding::encode_set(&[0x0f, 0x9c], opr1.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Setle => match opr1 {
            Operand::Register(_) | Operand::Memory(_) => {
                encoding::encode_set(&[0x0f, 0x9e], opr1.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Setne => match opr1 {
            Operand::Register(_) | Operand::Memory(_) => {
                encoding::encode_set(&[0x0f, 0x95], opr1.to_rm())
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}

// todo サイズの比較
fn encode_binary_op(inst: &Instruction) -> EncodedInst {
    let opr1 = inst.operand1.as_ref().expect("first operand is required");
    let opr2 = inst.operand2.as_ref().expect("second operand is required");

    match inst.mnenomic {
        Mnemonic::Add => match (opr1, opr2) {
            (Operand::Register(_), Operand::Immediate(imm))
            | (Operand::Memory(_), Operand::Immediate(imm)) => match imm {
                Immediate::Imm8(_) => encoding::encode_mi(&[0x83], opr1.to_rm(), imm).set_reg(0),
                Immediate::Imm32(_) => encoding::encode_mi(&[0x81], opr1.to_rm(), imm).set_reg(0),
            },
            (Operand::Register(_), Operand::Register(reg))
            | (Operand::Memory(_), Operand::Register(reg)) => {
                encoding::encode_mr(&[0x01], opr1.to_rm(), reg)
            }
            (Operand::Register(reg), Operand::Memory(_)) => {
                encoding::encode_rm(&[0x03], reg, opr2.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::And => match (opr1, opr2) {
            (Operand::Register(_), Operand::Immediate(imm))
            | (Operand::Memory(_), Operand::Immediate(imm)) => match imm {
                Immediate::Imm8(_) => encoding::encode_mi(&[0x83], opr1.to_rm(), imm).set_reg(4),
                Immediate::Imm32(_) => encoding::encode_mi(&[0x81], opr1.to_rm(), imm).set_reg(4),
            },
            (Operand::Register(_), Operand::Register(reg))
            | (Operand::Memory(_), Operand::Register(reg)) => {
                encoding::encode_mr(&[0x21], opr1.to_rm(), reg)
            }
            (Operand::Register(reg), Operand::Memory(_)) => {
                encoding::encode_rm(&[0x23], reg, opr2.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Cmp => match (opr1, opr2) {
            (Operand::Register(_), Operand::Immediate(imm))
            | (Operand::Memory(_), Operand::Immediate(imm)) => match imm {
                Immediate::Imm8(_) => encoding::encode_mi(&[0x83], opr1.to_rm(), imm).set_reg(7),
                Immediate::Imm32(_) => encoding::encode_mi(&[0x81], opr1.to_rm(), imm).set_reg(7),
            },
            (Operand::Register(_), Operand::Register(reg))
            | (Operand::Memory(_), Operand::Register(reg)) => {
                encoding::encode_mr(&[0x39], opr1.to_rm(), reg)
            }
            (Operand::Register(reg), Operand::Memory(_)) => {
                encoding::encode_rm(&[0x3b], reg, opr2.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::IMul => match (opr1, opr2) {
            (Operand::Register(reg), Operand::Immediate(imm)) => match imm {
                Immediate::Imm8(_) => encoding::encode_rmi(&[0x6b], reg, opr1.to_rm(), imm),
                Immediate::Imm32(_) => encoding::encode_rmi(&[0x69], reg, opr1.to_rm(), imm),
            },
            (Operand::Register(reg), Operand::Register(_))
            | (Operand::Register(reg), Operand::Memory(_)) => {
                encoding::encode_rm(&[0x0f, 0xaf], reg, opr2.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Lea => match (opr1, opr2) {
            (Operand::Register(reg), Operand::Memory(_)) => {
                encoding::encode_rm(&[0x8d], reg, opr2.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Mov => match (opr1, opr2) {
            (Operand::Register(_), Operand::Immediate(imm))
            | (Operand::Memory(_), Operand::Immediate(imm)) => match imm {
                Immediate::Imm8(_) => panic!(),
                Immediate::Imm32(_) => encoding::encode_mi(&[0xc7], opr1.to_rm(), imm).set_reg(0),
            },
            (Operand::Register(_), Operand::Register(reg))
            | (Operand::Memory(_), Operand::Register(reg)) => match reg.size() {
                register::Size::Byte => encoding::encode_mr(&[0x88], opr1.to_rm(), reg),
                _ => encoding::encode_mr(&[0x89], opr1.to_rm(), reg),
            },
            (Operand::Register(reg), Operand::Memory(_)) => {
                encoding::encode_rm(&[0x8b], reg, opr2.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Movsx => match (opr1, opr2) {
            (Operand::Register(reg), Operand::Memory(_)) => {
                encoding::encode_rm(&[0x0f, 0xbe], reg, opr2.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Or => match (opr1, opr2) {
            (Operand::Register(_), Operand::Immediate(imm))
            | (Operand::Memory(_), Operand::Immediate(imm)) => match imm {
                Immediate::Imm8(_) => encoding::encode_mi(&[0x83], opr1.to_rm(), imm).set_reg(1),
                Immediate::Imm32(_) => encoding::encode_mi(&[0x81], opr1.to_rm(), imm).set_reg(1),
            },
            (Operand::Register(_), Operand::Register(reg))
            | (Operand::Memory(_), Operand::Register(reg)) => {
                encoding::encode_mr(&[0x09], opr1.to_rm(), reg)
            }
            (Operand::Register(reg), Operand::Memory(_)) => {
                encoding::encode_rm(&[0x0b], reg, opr2.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Sub => match (opr1, opr2) {
            (Operand::Register(_), Operand::Immediate(imm))
            | (Operand::Memory(_), Operand::Immediate(imm)) => match imm {
                Immediate::Imm8(_) => encoding::encode_mi(&[0x83], opr1.to_rm(), imm).set_reg(5),
                Immediate::Imm32(_) => encoding::encode_mi(&[0x81], opr1.to_rm(), imm).set_reg(5),
            },
            (Operand::Register(_), Operand::Register(reg))
            | (Operand::Memory(_), Operand::Register(reg)) => {
                encoding::encode_mr(&[0x29], opr1.to_rm(), reg)
            }
            (Operand::Register(reg), Operand::Memory(_)) => {
                encoding::encode_rm(&[0x2b], reg, opr2.to_rm())
            }
            _ => panic!(),
        },
        Mnemonic::Xor => match (opr1, opr2) {
            (Operand::Register(_), Operand::Immediate(imm))
            | (Operand::Memory(_), Operand::Immediate(imm)) => match imm {
                Immediate::Imm8(_) => encoding::encode_mi(&[0x83], opr1.to_rm(), imm).set_reg(6),
                Immediate::Imm32(_) => encoding::encode_mi(&[0x81], opr1.to_rm(), imm).set_reg(6),
            },
            (Operand::Register(_), Operand::Register(reg))
            | (Operand::Memory(_), Operand::Register(reg)) => {
                encoding::encode_mr(&[0x31], opr1.to_rm(), reg)
            }
            (Operand::Register(reg), Operand::Memory(_)) => {
                encoding::encode_rm(&[0x33], reg, opr2.to_rm())
            }
            _ => panic!(),
        },
        _ => panic!(),
    }
}

// TODO
impl EncodedInst {
    fn set_reg(mut self, reg: u8) -> Self {
        self.modrm.as_mut().unwrap().reg = reg;
        self
    }
}

// TODO
impl Register {
    fn expect_size(&self, size: register::Size) -> &Self {
        if self.size() != size {
            panic!();
        }
        self
    }
}

// TODO
impl Operand {
    fn to_rm(&self) -> RM {
        match self {
            Operand::Register(reg) => RM::Register(reg),
            Operand::Memory(mem) => RM::Memory(mem),
            _ => panic!(),
        }
    }
}
