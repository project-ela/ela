use id_arena::{Arena, Id};

#[derive(Debug)]
pub struct Types {
    pub types: Arena<Type>,
}

impl Types {
    pub fn new() -> Self {
        Self {
            types: Arena::new(),
        }
    }

    pub fn ptr_to(&mut self, typ: Type) -> Type {
        let typ_id = self.types.alloc(typ);
        Type::Pointer(typ_id)
    }

    pub fn array_of(&mut self, typ: Type, len: usize) -> Type {
        let typ_id = self.types.alloc(typ);
        Type::Array(typ_id, len)
    }

    pub fn elm_typ(&self, typ: Type) -> Type {
        use self::Type::*;

        match typ {
            Pointer(typ_id) | Array(typ_id, _) => *self.types.get(typ_id).unwrap(),

            _ => panic!(),
        }
    }
}

pub type TypeId = Id<Type>;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Void,

    I1,
    I32,

    Pointer(TypeId),
    Array(TypeId, usize),
}

impl Type {
    pub fn size(&self, types: &Types) -> usize {
        use self::Type::*;

        match self {
            Void => 0,
            // TODO
            I1 => 8,
            I32 => 8,

            Pointer(_) => 8,
            Array(_, len) => types.elm_typ(*self).size(types) * len,
        }
    }
}
