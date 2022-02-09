use id_arena::Id;

use super::{Constant, Type};

pub type GlobalId = Id<Global>;

#[derive(Debug)]
pub struct Global {
    pub name: String,

    pub typ: Type,

    pub init_value: Constant,
}

impl Global {
    pub fn new<S: Into<String>>(name: S, typ: Type, init_value: Constant) -> Self {
        Self {
            name: name.into(),
            typ,
            init_value,
        }
    }
}
