#[derive(Debug, Clone)]
pub enum Type {
    Void,

    I1,
    I8,
    I32,

    Pointer(Box<Type>),
    Array(Box<Type>, usize),
}

impl Type {
    pub fn size(&self) -> usize {
        use self::Type::*;

        match self {
            Void => 0,
            I1 | I8 => 1,
            I32 => 8,

            Pointer(_) => 8,
            Array(elm_typ, len) => elm_typ.size() * len,
        }
    }

    pub fn elm_typ(&self) -> Type {
        use self::Type::*;

        match self {
            Pointer(typ) | Array(typ, _) => *typ.clone(),

            _ => panic!(),
        }
    }

    pub fn ptr_to(&self) -> Type {
        Type::Pointer(Box::new(self.clone()))
    }

    pub fn array_of(&self, len: usize) -> Type {
        Type::Array(Box::new(self.clone()), len)
    }
}
