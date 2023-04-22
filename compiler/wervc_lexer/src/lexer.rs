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

        let kind = match self.ch {
            _ if self.is_number() => {
                let literal = self.read_number();

                return Token::new(Number, literal);
            }
            _ if self.is_ident() => {
                let literal = self.read_ident();
                let kind = TokenKind::lookup_ident(&literal);

                return Token::new(kind, literal);
            }
            '=' if self.peek_char() == '=' => {
                self.read_char();
                self.read_char();
                return Token::new(Eq, "==");
            }
            '=' => Assign,
            '<' if self.peek_char() == '=' => {
                self.read_char();
                self.read_char();
                return Token::new(Le, "<=");
            }
            '<' => Lt,
            '>' if self.peek_char() == '=' => {
                self.read_char();
                self.read_char();
                return Token::new(Ge, ">=");
            }
            '>' => Gt,
            '!' if self.peek_char() == '=' => {
                self.read_char();
                self.read_char();
                return Token::new(Ne, "!=");
            }
            '!' => Bang,
            '+' => Plus,
            '-' => Minus,
            '*' => Asterisk,
            '/' => Slash,
            '&' => Ampersand,
            ';' => SemiColon,
            ',' => Comma,
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,
            '[' => LBracket,
            ']' => RBracket,
            '\0' => EOF,
            _ => Unknown,
        };
        let ch = self.ch;

        self.read_char();

        Token::new(kind, ch)
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

        while self.is_number() {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn is_number(&self) -> bool {
        self.ch.is_ascii_digit()
    }

    fn read_ident(&mut self) -> String {
        let position = self.position;

        while self.is_number() || self.is_ident() {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    fn is_ident(&self) -> bool {
        self.ch.is_ascii_alphabetic() || self.ch == '_'
    }
}
