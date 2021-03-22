use crate::common::pos::Pos;
use std::{error, fmt};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}:{1}")]
pub struct Error(Pos, anyhow::Error);

impl Error {
    pub fn new<T: Into<anyhow::Error>>(pos: Pos, err: T) -> Self {
        Self(pos, err.into())
    }
}

#[derive(Debug, Default)]
pub struct Errors(pub Vec<Error>);

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for err in self.0.iter() {
            writeln!(f, "{}", err)?;
        }
        Ok(())
    }
}

impl error::Error for Errors {}
