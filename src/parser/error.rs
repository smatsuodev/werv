use crate::token::TokenKind;

#[derive(Debug)]
pub enum ParseError {
    ParseParamsError,
    ParseBlockError,
    ParseConsumeError(TokenKind),
    ParseIntegerError,
    ParseArgsError,
}

use ParseError::*;
impl ToString for ParseError {
    fn to_string(&self) -> String {
        let body = match self {
            ParseParamsError => String::from("ParseParamsError"),
            ParseBlockError => String::from("ParseBlockError"),
            ParseConsumeError(kind) => format!("ParseConsumeError({:?})", kind),
            ParseIntegerError => String::from("ParseIntegerError"),
            ParseArgsError => String::from("ParseArgsError"),
        };

        format!("Parser Error: {}", body)
    }
}
