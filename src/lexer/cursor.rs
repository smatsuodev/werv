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

    pub(crate) fn second(&self) -> char {
        let mut iter = self.chars.clone();

        iter.next();

        iter.next().unwrap_or(EOF_CHAR)
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn eat_while(&mut self, mut cond: impl FnMut(char) -> bool) {
        while !self.is_eof() && cond(self.first()) {
            self.bump();
        }
    }

    fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.chars.as_str().len()) as u32
    }

    fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    fn eat_whitespace(&mut self) {
        self.eat_while(|c| c.is_whitespace());
    }

    pub fn advance_token(&mut self) -> Token {
        if self.is_eof() {
            return Token::new(TokenKind::Eof, 0);
        }

        self.eat_whitespace();
        self.reset_pos_within_token();

        let kind = match self.first() {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            ';' => TokenKind::SemiColon,
            ',' => TokenKind::Comma,

            '=' if self.second() == '=' => {
                self.bump();

                TokenKind::Eq
            }
            '=' => TokenKind::Assign,
            '!' if self.second() == '=' => {
                self.bump();

                TokenKind::Ne
            }
            '<' if self.second() == '=' => {
                self.bump();

                TokenKind::Le
            }
            '<' => TokenKind::Lt,
            '>' if self.second() == '=' => {
                self.bump();

                TokenKind::Ge
            }
            '>' => TokenKind::Gt,

            c => {
                if c.is_digit(10) {
                    self.bump();
                    self.eat_while(|c| c.is_digit(10));

                    return Token::new(TokenKind::Number, self.pos_within_token());
                } else {
                    TokenKind::Unknown
                }
            }
        };

        self.bump();

        Token::new(kind, self.pos_within_token())
    }
}
