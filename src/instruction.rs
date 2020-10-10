#[derive(Debug)]
pub enum Instruction {
    PseudoOp { name: String, arg: String },
    Label { name: String },
    NullaryOp(Opcode),
    UnaryOp(Opcode, Operand),
    BinaryOp(Opcode, Operand, Operand),
}

#[derive(Debug)]
pub enum Opcode {
    Push,
    Pop,
    Add,
    Sub,
    IMul,
    IDiv,
    Xor,
    Ret,
    Mov,
    Jmp,
    And,
    Or,
    Cmp,
}

#[derive(Debug)]
pub enum Operand {
    Immidiate { value: u32 },
    Register { reg: Register },
    Label { name: String },
}

#[derive(Debug)]
pub enum Register {
    Eax,
    Ecx,
    Edx,
    Ebx,
    Esp,
    Ebp,
    Esi,
    Edi,

    Al,
    Cl,
    Dl,
    Bl,
}

#[derive(Eq, PartialEq)]
pub enum RegSize {
    Byte,
    Word,
    DWord,
    QWord,
}

impl Register {
    pub fn size(&self) -> RegSize {
        use self::Register::*;
        match self {
            Eax | Ecx | Edx | Ebx | Esp | Ebp | Esi | Edi => RegSize::DWord,
            Al | Cl | Dl | Bl => RegSize::Byte,
        }
    }

    pub fn number(self) -> u8 {
        use self::Register::*;
        match self {
            Eax | Al => 0,
            Ecx | Cl => 1,
            Edx | Dl => 2,
            Ebx | Bl => 3,
            Esp => 4,
            Ebp => 5,
            Esi => 6,
            Edi => 7,
        }
    }
}
