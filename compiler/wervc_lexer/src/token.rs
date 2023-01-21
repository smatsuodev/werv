#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}
impl Token {
    pub fn new(kind: TokenKind, literal: impl ToString) -> Token {
        Token {
            kind,
            literal: literal.to_string(),
        }
    }
    pub fn kind(&self) -> TokenKind {
        self.kind
    }
    pub fn literal(&self) -> &str {
        &self.literal
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
    SemiColon,
}

impl Default for TokenKind {
    fn default() -> Self {
        TokenKind::EOF
    }
}
