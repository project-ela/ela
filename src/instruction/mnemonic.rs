#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mnemonic {
    Add,
    And,
    Call,
    Cmp,
    Hlt,
    IDiv,
    IMul,
    Je,
    Jmp,
    Mov,
    Or,
    Pop,
    Push,
    Ret,
    Sete,
    Setg,
    Setge,
    Setl,
    Setle,
    Setne,
    Sub,
    Syscall,
    Xor,
}

pub enum Type {
    Nullary,
    Unary,
    Binary,
}

impl Mnemonic {
    pub fn typ(&self) -> Type {
        use Mnemonic::*;
        use Type::*;
        match self {
            Add => Binary,
            And => Binary,
            Call => Unary,
            Cmp => Binary,
            Hlt => Nullary,
            IDiv => Unary,
            IMul => Binary,
            Je => Unary,
            Jmp => Unary,
            Mov => Binary,
            Or => Binary,
            Pop => Unary,
            Push => Unary,
            Ret => Nullary,
            Sete => Unary,
            Setg => Unary,
            Setge => Unary,
            Setl => Unary,
            Setle => Unary,
            Setne => Unary,
            Sub => Binary,
            Syscall => Nullary,
            Xor => Binary,
        }
    }
}
