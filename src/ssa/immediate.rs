use super::Type;

#[derive(Debug, Clone, Copy)]
pub enum Immediate {
    I32(i32),
}

impl Immediate {
    pub fn typ(&self) -> Type {
        use self::Immediate::*;

        match self {
            I32(_) => Type::I32,
        }
    }
}
