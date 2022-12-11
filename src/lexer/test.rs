use super::Lexer;
use crate::token::{Token, TokenKind::*};

#[test]
fn next_token_test() {
    let input = r"
let _aA10 = 1 + 2 - 3 * 4 / 5; 
fn fib(n) = fib(n-1) + fib(n-2);
fn add(a, b) = {
    return a + b;
};
fn only_odd(n) = if n % 2 { n } else { 0 };
if false { !false } else if true { !true }";
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
        (Fn, "fn"),
        (Ident, "fib"),
        (LParen, "("),
        (Ident, "n"),
        (RParen, ")"),
        (Assign, "="),
        (Ident, "fib"),
        (LParen, "("),
        (Ident, "n"),
        (Minus, "-"),
        (Number, "1"),
        (RParen, ")"),
        (Plus, "+"),
        (Ident, "fib"),
        (LParen, "("),
        (Ident, "n"),
        (Minus, "-"),
        (Number, "2"),
        (RParen, ")"),
        (SemiColon, ";"),
        (Fn, "fn"),
        (Ident, "add"),
        (LParen, "("),
        (Ident, "a"),
        (Comma, ","),
        (Ident, "b"),
        (RParen, ")"),
        (Assign, "="),
        (LBrace, "{"),
        (Return, "return"),
        (Ident, "a"),
        (Plus, "+"),
        (Ident, "b"),
        (SemiColon, ";"),
        (RBrace, "}"),
        (SemiColon, ";"),
        (Fn, "fn"),
        (Ident, "only_odd"),
        (LParen, "("),
        (Ident, "n"),
        (RParen, ")"),
        (Assign, "="),
        (If, "if"),
        (Ident, "n"),
        (Percent, "%"),
        (Number, "2"),
        (LBrace, "{"),
        (Ident, "n"),
        (RBrace, "}"),
        (Else, "else"),
        (LBrace, "{"),
        (Number, "0"),
        (RBrace, "}"),
        (SemiColon, ";"),
        (If, "if"),
        (False, "false"),
        (LBrace, "{"),
        (Bang, "!"),
        (False, "false"),
        (RBrace, "}"),
        (Else, "else"),
        (If, "if"),
        (True, "true"),
        (LBrace, "{"),
        (Bang, "!"),
        (True, "true"),
        (RBrace, "}"),
        (EOF, "\0"),
    ];
    let mut l = Lexer::new(input.to_string());

    for (kind, literal) in expect {
        assert_eq!(l.next_token(), Token::new(kind, literal.to_string()));
    }

    assert_eq!(l.next_token(), Token::new(EOF, "\0".to_string()));
}
