#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Unknown,
    EOF,

    Number,

    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}
