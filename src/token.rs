#[derive(Debug, PartialEq)]
pub enum TokenKind {
    /// =
    Assign,
    /// ;
    SemiColon,

    // キーワード
    Let,

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
