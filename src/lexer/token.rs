#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    /// 下記以外のトークン
    Unknown,
    /// 入力の終わり
    EOF,

    /// 数字
    Number,
    /// 識別子
    Ident,

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
    /// =>
    Arrow,

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
    /// ,
    Comma,
    /// 改行
    NewLine,
}
