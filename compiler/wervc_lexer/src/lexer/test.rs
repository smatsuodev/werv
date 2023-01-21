use crate::{
    lexer::Lexer,
    token::{
        Token,
        TokenKind::{self, *},
    },
};

fn loop_assert<const N: usize>(inputs: [impl ToString; N], expects: [Vec<(TokenKind, &str)>; N]) {
    for (input, expects) in inputs.into_iter().zip(expects) {
        let mut lexer = Lexer::new(input);

        for (kind, literal) in expects {
            assert_eq!(Token::new(kind, literal), lexer.next_token());
        }
    }
}

#[test]
fn lexer_number_test() {
    let inputs = ["0", "42", "1234567890;"];
    let expects = [
        vec![(Number, "0"), (EOF, "\0")],
        vec![(Number, "42"), (EOF, "\0")],
        vec![(Number, "1234567890"), (SemiColon, ";"), (EOF, "\0")],
    ];

    loop_assert(inputs, expects);
}

#[test]
fn lexer_arithmetic_test() {
    let inputs = ["1 + (2 - 3) * 4 / 5"];
    let expects = [vec![
        (Number, "1"),
        (Plus, "+"),
        (LParen, "("),
        (Number, "2"),
        (Minus, "-"),
        (Number, "3"),
        (RParen, ")"),
        (Asterisk, "*"),
        (Number, "4"),
        (Slash, "/"),
        (Number, "5"),
        (EOF, "\0"),
    ]];

    loop_assert(inputs, expects);
}

#[test]
fn lexer_let_expr_test() {
    let inputs = ["let x = 5;", "let y = 10"];
    let expects = [
        vec![
            (Let, "let"),
            (Ident, "x"),
            (Assign, "="),
            (Number, "5"),
            (SemiColon, ";"),
            (EOF, "\0"),
        ],
        vec![
            (Let, "let"),
            (Ident, "y"),
            (Assign, "="),
            (Number, "10"),
            (EOF, "\0"),
        ],
    ];

    loop_assert(inputs, expects);
}

#[test]
fn lexer_block_expr_test() {
    let inputs = ["{ 10 }", "{ let x = 10; let y = 20; x + y }"];
    let expects = [
        vec![(LBrace, "{"), (Number, "10"), (RBrace, "}"), (EOF, "\0")],
        vec![
            (LBrace, "{"),
            (Let, "let"),
            (Ident, "x"),
            (Assign, "="),
            (Number, "10"),
            (SemiColon, ";"),
            (Let, "let"),
            (Ident, "y"),
            (Assign, "="),
            (Number, "20"),
            (SemiColon, ";"),
            (Ident, "x"),
            (Plus, "+"),
            (Ident, "y"),
            (RBrace, "}"),
            (EOF, "\0"),
        ],
    ];

    loop_assert(inputs, expects);
}
