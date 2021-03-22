use thiserror::Error;

use crate::frontend::lexer::token::TokenKind;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("unexpected {0:?}")]
    UnexpectedToken(TokenKind),

    #[error("unexpected {0:?}, expecting {1:?}")]
    Expected(TokenKind, TokenKind),

    #[error("expected integer, but got {0:?}")]
    ExpectedInteger(TokenKind),

    #[error("expected identifier, but got {0:?}")]
    ExpectedIdent(TokenKind),

    #[error("'{0}' is not a type name")]
    NotTypeName(String),
}
