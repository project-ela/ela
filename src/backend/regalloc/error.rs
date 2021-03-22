use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegallocError {
    #[error("registers exhausted")]
    RegistersExhausted,
}
