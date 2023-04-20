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
    Ident,

    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Ampersand,

    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    SemiColon,
    Comma,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    Let,
    If,
    Else,
    True,
    False,
    Return,
}

impl Default for TokenKind {
    fn default() -> Self {
        TokenKind::EOF
    }
}

impl TokenKind {
    pub fn lookup_ident(literal: &str) -> TokenKind {
        match literal {
            "let" => Self::Let,
            "if" => Self::If,
            "else" => Self::Else,
            "true" => Self::True,
            "false" => Self::False,
            "return" => Self::Return,
            _ => Self::Ident,
        }
    }
}
