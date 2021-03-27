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

    pub fn elm_typ(&self, typ: Type) -> Type {
        use self::Type::*;

        match typ {
            Pointer(typ_id) => *self.types.get(typ_id).unwrap(),
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
}
