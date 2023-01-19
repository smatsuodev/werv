use crate::token::TokenKind::{self, *};

#[cfg(test)]
mod test;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: impl ToString) -> Lexer {
        Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
            self.position = self.read_position;
            self.read_position += 1;
        }
    }

    pub fn next_token(&mut self) -> TokenKind {
        match self.ch {
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '/' => Slash,
            '\0' => EOF,
            c if c.is_digit(10) => self.read,
            c => Unknown(c),
        }
    }
}
