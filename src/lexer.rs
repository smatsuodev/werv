#[cfg(test)]
mod test;
mod token;

use self::token::{Token, TokenKind};
use std::str::Chars;

const EOF_CHAR: char = '\0';

fn is_newline(ch: char) -> bool {
    ch == '\n'
}

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

    /// いま着目している文字を返す
    fn second(&self) -> char {
        let mut chars = self.chars.clone();

        // chars.nth(1)よりもchars.next()を2回呼ぶ方がパフォーマンスがいいらしい
        // 参考: https://github.com/rust-lang/rust/blob/master/compiler/rustc_lexer/src/cursor.rs#L52
        chars.next();

        chars.next().unwrap_or(EOF_CHAR)
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

    /// 空白を無視する
    fn eat_whitespace(&mut self) {
        self.eat_while(|c| c.is_whitespace() && !is_newline(c));
    }

    /// 次の文字が`ch`なら文字を1文字読み進めて真を返す
    /// それ以外なら偽を返す
    fn is_peek(&mut self, ch: char) -> bool {
        if self.second() == ch {
            self.bump();

            true
        } else {
            false
        }
    }

    /// 次のトークンを返す
    pub fn next_token(&mut self) -> Token {
        if self.is_eof() {
            return Token::new(TokenKind::EOF, 0);
        }

        self.eat_whitespace();
        self.reset_pos_within_token();

        let kind = match self.first() {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '!' if self.is_peek('=') => TokenKind::Ne,
            '!' => TokenKind::Bang,
            '=' if self.is_peek('>') => TokenKind::Arrow,
            '=' if self.is_peek('=') => TokenKind::Eq,
            '=' => TokenKind::Assign,
            '<' if self.is_peek('=') => TokenKind::Le,
            '<' => TokenKind::Lt,
            '>' if self.is_peek('=') => TokenKind::Ge,
            '>' => TokenKind::Gt,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            ',' => TokenKind::Comma,
            c if is_newline(c) => TokenKind::NewLine,
            c if c.is_digit(10) => {
                self.eat_while(|c| c.is_digit(10));

                return Token::new(TokenKind::Number, self.pos_within_token());
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                self.eat_while(|c| c.is_ascii_alphabetic() || c.is_digit(10) || c == '_');

                return Token::new(TokenKind::Ident, self.pos_within_token());
            }
            _ => TokenKind::Unknown,
        };

        self.bump();

        Token::new(kind, self.pos_within_token())
    }
}
