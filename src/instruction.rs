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
}

impl Into<u8> for Register {
    fn into(self) -> u8 {
        self as u8
    }
}
