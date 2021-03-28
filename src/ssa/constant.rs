use super::Type;

#[derive(Debug, Clone, Copy)]
pub enum Constant {
    I32(i32),
}

impl Constant {
    pub fn typ(&self) -> Type {
        use self::Constant::*;

        match self {
            I32(_) => Type::I32,
        }
    }
}
