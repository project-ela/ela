use super::Type;

#[derive(Debug, Clone, Copy)]
pub enum Constant {
    I1(bool),
    I8(i8),
    I32(i32),
}

impl Constant {
    pub fn typ(&self) -> Type {
        use self::Constant::*;

        match self {
            I1(_) => Type::I1,
            I8(_) => Type::I8,
            I32(_) => Type::I32,
        }
    }
}
