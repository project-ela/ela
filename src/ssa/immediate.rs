use std::fmt;

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

impl fmt::Display for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Immediate::*;

        match self {
            I32(x) => write!(f, "{} {}", self.typ(), x),
        }
    }
}
