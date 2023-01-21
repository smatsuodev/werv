use std::num::ParseIntError;
use wervc_lexer::token::TokenKind;

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken { expected: TokenKind, got: TokenKind },
    ParseIntError(ParseIntError),
}
