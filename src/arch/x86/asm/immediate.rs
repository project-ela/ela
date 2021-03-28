use crate::ssa;

#[derive(Debug)]
pub enum Immediate {
    I8(i8),
    I32(i32),
}

impl From<&ssa::Constant> for Immediate {
    fn from(r#const: &ssa::Constant) -> Self {
        use ssa::Constant::*;

        match r#const {
            I1(x) => Self::I8(*x as i8),
            I32(x) => Self::I32(*x),
        }
    }
}

impl Immediate {
    pub fn stringify(&self) -> String {
        use self::Immediate::*;

        match self {
            I8(x) => format!("{}", x),
            I32(x) => format!("{}", x),
        }
    }
}
