use crate::token::TokenKind;

#[derive(Debug)]
pub enum ParseError {
    ParseParamsError,
    ParseBlockError,
    ParseConsumeError(TokenKind),
    ParseIntegerError,
    ParseArgsError,
}
