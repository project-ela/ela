use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Void,
    Int,
    Bool,
    Pointer { pointer_to: Box<Type> },
    Array { elm_type: Box<Type>, len: u32 },
}

impl Type {
    pub fn size(&self) -> u32 {
        match self {
            Type::Void => 8,
            Type::Int => 8,
            Type::Bool => 8,
            Type::Pointer { .. } => 8,
            Type::Array { elm_type, len } => elm_type.size() * len,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::Pointer { pointer_to } => write!(f, "*{}", pointer_to),
            Type::Array { elm_type, len } => write!(f, "{}[{}]", elm_type, len),
        }
    }
}
