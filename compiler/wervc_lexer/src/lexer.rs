use crate::token::{
    Token,
    TokenKind::{self, *},
};

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

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();

        let ch = self.ch;
        let mut literal = ch.to_string();
        let kind = match ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = format!("{ch}=");

                    Eq
                } else {
                    Assign
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = format!("{ch}=");

                    Le
                } else {
                    Lt
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = format!("{ch}=");

                    Ge
                } else {
                    Gt
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    literal = format!("{ch}=");

                    Ne
                } else {
                    Bang
                }
            }
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '/' => Slash,
            ';' => SemiColon,
            ',' => Comma,
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,
            '[' => LBracket,
            ']' => RBracket,
            '\0' => EOF,
            c if Lexer::is_digit(c) => {
                let literal = self.read_number();

                return Token::new(Number, literal);
            }
            c if Lexer::is_ident_char(c) => {
                let literal = self.read_ident();
                let kind = TokenKind::lookup_ident(&literal);

                return Token::new(kind, literal);
            }
            _ => Unknown,
        };

        self.read_char();

        Token::new(kind, literal)
    }

    fn eat_whitespace(&mut self) {
        while self.is_whitespace() {
            self.read_char();
        }
    }

    fn is_whitespace(&self) -> bool {
        self.ch.is_whitespace()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;

        while Lexer::is_digit(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    fn read_ident(&mut self) -> String {
        let position = self.position;

        while Lexer::is_ident_char(self.ch) || Lexer::is_digit(self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn is_ident_char(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }
}
