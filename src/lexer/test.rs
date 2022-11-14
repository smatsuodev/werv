use crate::lexer::token::{Token, TokenKind::*};

use super::Lexer;

#[test]
fn test_number() {
    let tests = ["0", "123"];

    for input in tests {
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::new(Number, input.len()));
        assert_eq!(lexer.next_token(), Token::new(EOF, 0));
    }
}

#[test]
fn test_arithmetic() {
    let input = "(1 + (22 - 333) * 4444) / 55555";
    let expects = [
        (LParen, 1),
        (Number, 1),
        (Plus, 1),
        (LParen, 1),
        (Number, 2),
        (Minus, 1),
        (Number, 3),
        (RParen, 1),
        (Asterisk, 1),
        (Number, 4),
        (RParen, 1),
        (Slash, 1),
        (Number, 5),
        (EOF, 0),
    ];
    let mut lexer = Lexer::new(input);

    for (kind, len) in expects {
        assert_eq!(lexer.next_token(), Token::new(kind, len));
    }
}
