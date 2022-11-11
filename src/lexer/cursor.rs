use super::token::{Token, TokenKind};
use std::str::Chars;

const EOF_CHAR: char = '\0';

pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len_remaining: input.len(),
            chars: input.chars(),
        }
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub(crate) fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn eat_while(&mut self, mut cond: impl FnMut(char) -> bool) {
        while !self.is_eof() && cond(self.first()) {
            self.bump();
        }
    }

    fn eat_whitespace(&mut self) {
        self.eat_while(|c| c.is_whitespace());
    }

    pub fn advance_token(&mut self) -> Token {
        if self.is_eof() {
            return Token::new(TokenKind::Eof, 0);
        }

        self.eat_whitespace();

        let mut len = 1;
        let kind = match self.first() {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            c => {
                if c.is_digit(10) {
                    self.bump();
                    self.eat_while(|c| {
                        if c.is_digit(10) {
                            len += 1;

                            true
                        } else {
                            false
                        }
                    });

                    TokenKind::Number
                } else {
                    TokenKind::Unknown
                }
            }
        };

        if len == 1 {
            self.bump();
        }

        Token::new(kind, len)
    }
}
