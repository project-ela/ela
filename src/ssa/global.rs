use id_arena::Id;

use super::Type;

pub type GlobalId = Id<Global>;

#[derive(Debug)]
pub struct Global {
    pub name: String,

    pub typ: Type,
}

impl Global {
    pub fn new(name: &str, typ: Type) -> Self {
        Self {
            name: name.into(),
            typ,
        }
    }
}
