use crate::token::{Token, TokenKind};
use std::str::Chars;

#[cfg(test)]
mod test;

const EOF_CHAR: char = '\0';

pub struct Lexer<'a> {
    chars: Chars<'a>,
    len_remaining: usize,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars(),
            len_remaining: input.len(),
        }
    }

    /// 1文字進める
    fn bump(&mut self) {
        self.chars.next();
    }

    /// いま着目している文字を返す
    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// 入力文字列を全てトークナイズし終わったなら真を、
    /// そうでないなら偽を返す
    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// 現在のトークンが占める文字の個数を返す
    fn pos_within_token(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    /// 入力文字列の残り文字数を現在のトークンの位置でリセットする
    fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.chars.as_str().len()
    }

    /// 条件が真の間、文字を読み進める
    fn eat_while(&mut self, cond: impl Fn(char) -> bool) {
        while !self.is_eof() && cond(self.first()) {
            self.bump();
        }
    }

    fn eat_whitespaces(&mut self) {
        self.eat_while(|c| c.is_whitespace());
    }

    /// 次のトークンを返す
    pub fn next_token(&mut self) -> Token {
        if self.is_eof() {
            return Token::new(TokenKind::EOF, 0);
        }

        self.eat_whitespaces();
        self.reset_pos_within_token();

        let kind = match self.first() {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            c if c.is_digit(10) => {
                self.eat_while(|c| c.is_digit(10));

                return Token::new(TokenKind::Number, self.pos_within_token());
            }
            _ => TokenKind::Unknown,
        };

        self.bump();

        Token::new(kind, self.pos_within_token())
    }
}
