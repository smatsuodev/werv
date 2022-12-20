#[derive(Debug)]
pub enum LexerError {
    LexerReadStringError,
}

use LexerError::*;
impl ToString for LexerError {
    fn to_string(&self) -> String {
        let body = match self {
            LexerReadStringError => "LexerReadStringError",
        };

        format!("Lexer Error: {body}")
    }
}
