#[derive(Debug, Clone)]
pub enum Type {
    Void,

    I1,
    I8,
    I32,

    Pointer(Box<Type>),
    Array(Box<Type>, usize),
    Structure(StructType),
}

#[derive(Debug, Clone)]
pub struct StructType {
    pub members: Vec<Type>,
}

impl Type {
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
