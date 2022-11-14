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
    /// 下記以外のトークン
    Unknown,
    /// 入力の終わり
    EOF,

    /// 数字
    Number,

    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,

    /// !
    Bang,

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

    /// (
    LParen,
    /// )
    RParen,
}
