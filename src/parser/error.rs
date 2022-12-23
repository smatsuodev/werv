use crate::{lexer::error::LexerError, token::TokenKind};

#[derive(Debug)]
pub enum ParseError {
    ParseBlockExprError,
    ParseConsumeError { expected: TokenKind, got: TokenKind },
    ParseIntegerError,
    ParseNextTokenError(LexerError),
    ParseEscapeError,
    ParseCallExprError,
    ParseArrayExprError,
}

use ParseError::*;
impl ToString for ParseError {
    fn to_string(&self) -> String {
        let body = match self {
            ParseBlockExprError => String::from("ParseBlockExprError"),
            ParseConsumeError { expected, got } => {
                format!(
                    "ParseConsumeError(expected {:?}, but got {:?})",
                    expected, got
                )
            }
            ParseIntegerError => String::from("ParseIntegerError"),
            ParseNextTokenError(e) => e.to_string(),
            ParseEscapeError => String::from("ParseEscapeError"),
            ParseCallExprError => String::from("ParseCallExprError"),
            ParseArrayExprError => String::from("ParseArrayExprError"),
        };

        format!("Parser Error: {body}")
    }
}
