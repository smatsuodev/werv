#[cfg(test)]
mod test;
use crate::token::{Token, TokenKind};

pub struct Lexer {
    input: String,
    position: usize,
    next_position: usize,
    ch: char,
}
impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            next_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let c = self.ch;
        let mut literal = c.to_string();
        let kind = match c {
            '"' => TokenKind::DoubleQuote,
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = format!("{}=", c);

                    TokenKind::Le
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = format!("{}=", c);

                    TokenKind::Ge
                } else {
                    TokenKind::Gt
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = format!("{}=", c);

                    TokenKind::Eq
                } else {
                    TokenKind::Assign
                }
            }
            ',' => TokenKind::Comma,
            ';' => TokenKind::SemiColon,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = format!("{}=", c);

                    TokenKind::Ne
                } else {
                    TokenKind::Bang
                }
            }
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '\0' => TokenKind::EOF,
            _ if self.is_number() => {
                return Token::new(TokenKind::Number, self.read_number());
            }
            _ if self.is_ident_head() => {
                let literal = self.read_ident();

                return Token::new(TokenKind::lookup_kind(&literal), literal);
            }
            _ => TokenKind::Unknown,
        };

        self.read_char();
        Token::new(kind, literal)
    }

    fn skip_whitespace(&mut self) {
        while self.is_whitespace() {
            self.read_char();
        }
    }

    fn is_whitespace(&self) -> bool {
        let c = self.ch;
        c == ' ' || c == '\t' || c == '\r' || c == '\n'
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while self.is_ident() {
            self.read_char();
        }
        self.input[pos..self.position].to_string()
    }

    fn is_ident(&self) -> bool {
        self.is_ident_head() || self.is_number()
    }

    fn is_ident_head(&self) -> bool {
        let c = self.ch;
        'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.is_number() {
            self.read_char();
        }
        self.input[pos..self.position].to_string()
    }

    fn is_number(&self) -> bool {
        '0' <= self.ch && self.ch <= '9'
    }

    fn read_char(&mut self) {
        if self.next_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.next_position).unwrap();
        }
        self.position = self.next_position;
        self.next_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.next_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.next_position).unwrap()
        }
    }
}
