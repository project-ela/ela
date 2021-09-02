use thiserror::Error;

use crate::common::{
    operator::{BinaryOperator, UnaryOperator},
    types::Type,
};

#[derive(Debug, Error)]
pub enum PassError {
    #[error("there must be 'main' function")]
    MainNotFound,

    #[error("'main' function should return int value")]
    MainShouldReturnInt,

    #[error("type mismatch {0} and {1}")]
    TypeMismatch(Type, Type),

    #[error("cannot index type {0}")]
    CannotIndex(Type),

    #[error("cannot load type {0}")]
    CannotLoad(Type),

    #[error("cannot assign to constant variable '{0}'")]
    AssignToConstant(String),

    #[error("undefined variable '{0}'")]
    NotDefinedVariable(String),

    #[error("undefined function '{0}'")]
    NotDefinedFunction(String),

    #[error("cannot {0:?} {1}")]
    UnaryOpErr(UnaryOperator, Type),

    #[error("cannot {1} {0:?} {2}")]
    BinaryOpErr(BinaryOperator, Type, Type),

    #[error("'{0}' function takes {1} arguments but {2} arguments were supplied")]
    FunctionArgNum(String, usize, usize),

    #[error("redefinition of '{0}'")]
    RedefinitionOf(String),

    #[error("lvalue required")]
    LvalueRequired,
}
