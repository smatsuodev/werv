#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    len: u32,
}

impl Token {
    pub fn new(kind: TokenKind, len: u32) -> Token {
        Token { kind, len }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,

    /// =
    Assign,

    /// ==
    Eq,
    /// !=
    Ne,
    /// <
    Lt,
    /// <=
    Le,
    /// >
    Gt,
    /// >=
    Ge,

    /// デリミタ
    SemiColon,
    Comma,

    /// (
    LParen,
    /// )
    RParen,

    /// 数字
    Number,

    /// 識別子
    /// (a-z|A-Z|_)+
    Ident,

    /// 上記以外のトークン
    Unknown,

    /// 入力終了
    Eof,
}
