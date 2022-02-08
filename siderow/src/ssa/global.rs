use id_arena::Id;

use super::{Constant, Type};

pub type GlobalId = Id<Global>;

#[derive(Debug)]
pub struct Global {
    pub name: String,

    pub init_value: Constant,

    pub typ: Type,
}

impl Global {
    pub fn new<S: Into<String>>(name: S, init_value: Constant) -> Self {
        Self {
            name: name.into(),
            typ: init_value.typ(),
            init_value,
        }
    }
}
