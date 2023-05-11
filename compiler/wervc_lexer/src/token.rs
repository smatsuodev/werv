#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}
impl Token {
    pub fn new(kind: TokenKind, literal: impl ToString) -> Token {
        Token {
            kind,
            literal: literal.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum TokenKind {
    Unknown,
    #[default]
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

    Colon,
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
