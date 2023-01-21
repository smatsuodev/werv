use std::num::ParseIntError;
use wervc_lexer::token::TokenKind;

#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
    UnexpectedToken { expected: TokenKind, got: TokenKind },
    ParseIntError(ParseIntError),
    RequiredSemiColon,
}
