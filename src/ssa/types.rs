use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    I1,
    I32,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Type::*;

        match self {
            I1 => write!(f, "i1"),
            I32 => write!(f, "i32"),
        }
    }
}
