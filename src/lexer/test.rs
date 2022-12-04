use super::Lexer;
use crate::token::{Token, TokenKind::*};

#[test]
fn next_token_test() {
    let input = "let _aA10 = 1 + 2 - 3 * 4 / 5;";
    let expect = [
        (Let, "let"),
        (Ident, "_aA10"),
        (Assign, "="),
        (Number, "1"),
        (Plus, "+"),
        (Number, "2"),
        (Minus, "-"),
        (Number, "3"),
        (Asterisk, "*"),
        (Number, "4"),
        (Slash, "/"),
        (Number, "5"),
        (SemiColon, ";"),
        (EOF, "\0"),
    ];
    let mut l = Lexer::new(input.to_string());

    for (kind, literal) in expect {
        assert_eq!(l.next_token(), Token::new(kind, literal.to_string()));
    }

    assert_eq!(l.next_token(), Token::new(EOF, "\0".to_string()));
}
