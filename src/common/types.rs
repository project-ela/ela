use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Int,
    Bool,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
        }
    }
}
