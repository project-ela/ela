use super::Type;

#[derive(Debug, Clone)]
pub enum Constant {
    ZeroInitializer,

    I1(bool),
    I8(i8),
    I32(i32),

    Array(Vec<Constant>),
}

impl Constant {
    pub fn new_zero() -> Self {
        Self::ZeroInitializer
    }

    pub fn new_i1(val: bool) -> Self {
        Self::I1(val)
    }

    pub fn new_i8(val: i8) -> Self {
        Self::I8(val)
    }

    pub fn new_i32(val: i32) -> Self {
        Self::I32(val)
    }

    pub fn new_array(elems: Vec<Constant>) -> Self {
        Self::Array(elems)
    }

    pub fn new_array_from_bytes(bytes: &[u8]) -> Self {
        let bytes = bytes.iter().map(|byte| Self::I8(*byte as i8)).collect();
        Self::Array(bytes)
    }

    pub fn typ(&self) -> Type {
        use self::Constant::*;

        match self {
            // TODO
            ZeroInitializer => Type::Void,

            I1(_) => Type::I1,
            I8(_) => Type::I8,
            I32(_) => Type::I32,

            Array(elems) => Type::Array(Box::new(elems[0].typ()), elems.len()),
        }
    }
}
