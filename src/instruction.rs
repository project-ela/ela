#[derive(Debug)]
pub enum Instruction {
    PseudoOp { name: String, arg: String },
    Label { name: String },
    NullaryOp(Mnemonic),
    UnaryOp(Mnemonic, Operand),
    BinaryOp(Mnemonic, Operand, Operand),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Mnemonic {
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
    Sete,
    Je,
    Setne,
    Setl,
    Setle,
    Setg,
    Setge,
    Call,
}

#[derive(Debug)]
pub enum MnemonicType {
    Nullary,
    Unary,
    Binary,
}

impl Mnemonic {
    pub fn typ(&self) -> MnemonicType {
        use Mnemonic::*;
        use MnemonicType::*;
        match self {
            Ret => Nullary,
            Push | Pop | IDiv | Jmp | Sete | Je | Setne | Setl | Setle | Setg | Setge | Call => {
                Unary
            }
            Add | Sub | IMul | Xor | Mov | And | Or | Cmp => Binary,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operand {
    Immidiate { value: u32 },
    Register { reg: Register },
    Label { name: String },
    Address(Address),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rdi,
    Rsi,
    Rbp,
    Rsp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,

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

    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b,
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
            Rax | Rcx | Rdx | Rbx | Rsp | Rbp | Rsi | Rdi | R8 | R9 | R10 | R11 | R12 | R13
            | R14 | R15 => RegSize::QWord,
            Eax | Ecx | Edx | Ebx | Esp | Ebp | Esi | Edi => RegSize::DWord,
            Al | Cl | Dl | Bl | R8b | R9b | R10b | R11b | R12b | R13b | R14b | R15b => {
                RegSize::Byte
            }
        }
    }

    pub fn number(&self) -> u8 {
        use self::Register::*;
        match self {
            Rax | R8 | Eax | Al | R8b => 0,
            Rcx | R9 | Ecx | Cl | R9b => 1,
            Rdx | R10 | Edx | Dl | R10b => 2,
            Rbx | R11 | Ebx | Bl | R11b => 3,
            Rsp | R12 | Esp | R12b => 4,
            Rbp | R13 | Ebp | R13b => 5,
            Rsi | R14 | Esi | R14b => 6,
            Rdi | R15 | Edi | R15b => 7,
        }
    }

    pub fn only_in_64bit(&self) -> bool {
        use self::Register::*;
        matches!(
            self,
            R8 | R9
                | R10
                | R11
                | R12
                | R13
                | R14
                | R15
                | R8b
                | R9b
                | R10b
                | R11b
                | R12b
                | R13b
                | R14b
                | R15b
        )
    }
}

#[derive(Debug, Clone)]
pub struct Address {
    pub base: Register,
    pub disp: Option<i32>,
}
