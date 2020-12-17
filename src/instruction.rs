use mnemonic::Mnemonic;
use operand::Operand;

pub mod mnemonic;
pub mod operand;

#[derive(Debug)]
pub struct Instruction {
    pub mnenomic: Mnemonic,
    pub operand1: Option<Operand>,
    pub operand2: Option<Operand>,
}

impl Instruction {
    pub fn new_nullary(mnenomic: Mnemonic) -> Self {
        Self {
            mnenomic,
            operand1: None,
            operand2: None,
        }
    }

    pub fn new_unary(mnenomic: Mnemonic, operand1: Operand) -> Self {
        Self {
            mnenomic,
            operand1: Some(operand1),
            operand2: None,
        }
    }

    pub fn new_binary(mnenomic: Mnemonic, operand1: Operand, operand2: Operand) -> Self {
        Self {
            mnenomic,
            operand1: Some(operand1),
            operand2: Some(operand2),
        }
    }
}
