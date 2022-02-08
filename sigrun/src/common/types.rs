use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Void,
    Byte,
    Int,
    Bool,
    Pointer { pointer_to: Box<Type> },
    Array { elm_type: Box<Type>, len: u32 },
}

impl Type {
    pub fn size(&self) -> u32 {
        match self {
            Type::Void => 8,
            Type::Byte => 1,
            Type::Int => 8,
            Type::Bool => 1,
            Type::Pointer { .. } => 8,
            Type::Array { elm_type, len } => elm_type.size() * len,
        }
    }

    pub fn is_same(&self, other: &Type) -> bool {
        match self {
            Type::Byte | Type::Int => matches!(other, Type::Byte | Type::Int),
            Type::Pointer { pointer_to: typ1 } | Type::Array { elm_type: typ1, .. } => {
                match other {
                    Type::Pointer { pointer_to: typ2 } | Type::Array { elm_type: typ2, .. } => {
                        typ1 == typ2
                    }
                    _ => false,
                }
            }
            _ => self == other,
        }
    }

    pub fn pointer_to(&self) -> Type {
        Type::Pointer {
            pointer_to: Box::new(self.clone()),
        }
    }

    pub fn elm_typ(&self) -> Type {
        match self {
            Type::Pointer { pointer_to } => *pointer_to.clone(),
            Type::Array { elm_type, len: _ } => *elm_type.clone(),
            _ => panic!(),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Void => write!(f, "void"),
            Type::Byte => write!(f, "byte"),
            Type::Int => write!(f, "int"),
            Type::Bool => write!(f, "bool"),
            Type::Pointer { pointer_to } => write!(f, "*{}", pointer_to),
            Type::Array { elm_type, len } => write!(f, "{}[{}]", elm_type, len),
        }
    }
}
