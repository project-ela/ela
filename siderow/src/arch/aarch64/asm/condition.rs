use std::fmt::Write;

use crate::ssa;

use super::Printer;

#[derive(Debug, Clone)]
pub enum Condition {
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
}

impl From<&ssa::ComparisonOperator> for Condition {
    fn from(value: &ssa::ComparisonOperator) -> Self {
        use ssa::ComparisonOperator::*;

        match value {
            Eq => Self::Eq,
            Neq => Self::Ne,
            Gt => Self::Gt,
            Gte => Self::Ge,
            Lt => Self::Lt,
            Lte => Self::Le,
        }
    }
}

impl Printer for Condition {
    fn print(&self, buf: &mut String) -> super::Result {
        use self::Condition::*;

        match self {
            Eq => write!(buf, "eq"),
            Ne => write!(buf, "ne"),
            Gt => write!(buf, "gt"),
            Lt => write!(buf, "lt"),
            Ge => write!(buf, "ge"),
            Le => write!(buf, "le"),
        }
    }
}
