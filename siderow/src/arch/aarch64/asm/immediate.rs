use std::fmt::Write;

use crate::ssa;

use super::Printer;

#[derive(Debug, Clone)]
pub enum Immediate {
    I32(i32),
}

impl From<&ssa::Constant> for Immediate {
    fn from(value: &ssa::Constant) -> Self {
        use ssa::Constant::*;

        match value {
            I32(value) => Self::I32(*value),
            _ => unimplemented!(),
        }
    }
}

impl Printer for Immediate {
    fn print(&self, buf: &mut String) -> super::Result {
        use self::Immediate::*;

        match self {
            I32(value) => write!(buf, "{}", value),
        }
    }
}
