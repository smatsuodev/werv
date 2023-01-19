pub enum TokenKind {
    Unknown(char),
    EOF,
    Number(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
}
