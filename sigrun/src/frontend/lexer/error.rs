use thiserror::Error;

#[derive(Debug, Error)]
pub enum LexerError {
    #[error("unexpected char '{0}'")]
    UnexpectedChar(char),
}
