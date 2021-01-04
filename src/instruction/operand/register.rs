#[derive(Debug, Clone, PartialEq, Eq)]
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
    Dil,
    Sil,
    Bpl,
    Spl,
    R8b,
    R9b,
    R10b,
    R11b,
    R12b,
    R13b,
    R14b,
    R15b,
}

#[derive(PartialEq, Eq)]
pub enum Size {
    QWord,
    DWord,
    Word,
    Byte,
}

impl Register {
    pub fn size(&self) -> Size {
        use self::Register::*;
        match self {
            Rax | Rcx | Rdx | Rbx | Rsp | Rbp | Rsi | Rdi | R8 | R9 | R10 | R11 | R12 | R13
            | R14 | R15 => Size::QWord,
            Eax | Ecx | Edx | Ebx | Esp | Ebp | Esi | Edi => Size::DWord,
            Al | Cl | Dl | Bl | Spl | Bpl | Sil | Dil | R8b | R9b | R10b | R11b | R12b | R13b
            | R14b | R15b => Size::Byte,
        }
    }

    pub fn number(&self) -> u8 {
        use Register::*;
        match self {
            Rax | R8 | Eax | Al | R8b => 0,
            Rcx | R9 | Ecx | Cl | R9b => 1,
            Rdx | R10 | Edx | Dl | R10b => 2,
            Rbx | R11 | Ebx | Bl | R11b => 3,
            Rsp | R12 | Esp | Spl | R12b => 4,
            Rbp | R13 | Ebp | Bpl | R13b => 5,
            Rsi | R14 | Esi | Sil | R14b => 6,
            Rdi | R15 | Edi | Dil | R15b => 7,
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
