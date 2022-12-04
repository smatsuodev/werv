#[derive(Debug, PartialEq)]
pub enum TokenKind {
    /// =
    Assign,
    /// ;
    SemiColon,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    /// (
    LParen,
    /// )
    RParen,
    /// {
    LBrace,
    /// }
    RBrace,

    // キーワード
    Let,
    Fn,
    Return,

    /// 識別子
    Ident,
    /// 数字
    Number,

    Unknown,
    EOF,
}
impl TokenKind {
    pub fn lookup_kind(literal: &str) -> TokenKind {
        match literal {
            "let" => TokenKind::Let,
            "fn" => TokenKind::Fn,
            "return" => TokenKind::Return,
            _ => TokenKind::Ident,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    literal: String,
}
impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Token {
        Token { kind, literal }
    }
}
