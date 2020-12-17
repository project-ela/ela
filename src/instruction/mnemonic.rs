#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mnemonic {
    Add,
    And,
    Call,
    Cmp,
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
            Xor => Binary,
        }
    }
}
