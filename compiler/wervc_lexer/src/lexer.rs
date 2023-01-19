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
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lexer.read_char();

        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> TokenKind {
        self.eat_whitespace();

        let token = match self.ch {
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '/' => Slash,
            '\0' => EOF,
            c if c.is_digit(10) => {
                let num = self.read_number();

                return TokenKind::Number(num);
            }
            c => Unknown(c),
        };

        self.read_char();
        token
    }

    pub fn eat_whitespace(&mut self) {
        while self.is_whitespace() {
            self.read_char();
        }
    }

    pub fn is_whitespace(&self) -> bool {
        self.ch.is_whitespace()
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;

        while self.is_digit() {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    pub fn is_digit(&mut self) -> bool {
        self.ch.is_digit(10)
    }
}
