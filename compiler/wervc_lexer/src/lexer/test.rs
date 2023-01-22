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

#[test]
fn lexer_assign_expr_test() {
    let inputs = ["x = 10;"];
    let expects = [vec![
        (Ident, "x"),
        (Assign, "="),
        (Number, "10"),
        (SemiColon, ";"),
        (EOF, "\0"),
    ]];

    loop_assert(inputs, expects);
}

#[test]
fn lexer_call_expr_test() {
    let inputs = ["add(1, 2 * 3, 4 + 5);"];
    let expects = [vec![
        (Ident, "add"),
        (LParen, "("),
        (Number, "1"),
        (Comma, ","),
        (Number, "2"),
        (Asterisk, "*"),
        (Number, "3"),
        (Comma, ","),
        (Number, "4"),
        (Plus, "+"),
        (Number, "5"),
        (RParen, ")"),
        (SemiColon, ";"),
        (EOF, "\0"),
    ]];

    loop_assert(inputs, expects);
}

#[test]
fn lexer_if_expr_test() {
    let inputs = ["if (x < y) { x } else { y }", "if true false"];
    let expects = [
        vec![
            (If, "if"),
            (LParen, "("),
            (Ident, "x"),
            (Lt, "<"),
            (Ident, "y"),
            (RParen, ")"),
            (LBrace, "{"),
            (Ident, "x"),
            (RBrace, "}"),
            (Else, "else"),
            (LBrace, "{"),
            (Ident, "y"),
            (RBrace, "}"),
            (EOF, "\0"),
        ],
        vec![(If, "if"), (True, "true"), (False, "false"), (EOF, "\0")],
    ];

    loop_assert(inputs, expects);
}

#[test]
fn lexer_relation_test() {
    let inputs = ["1 < 2", "1 > 2", "1 == 2", "1 != 2", "1 <= 2", "1 >= 2"];
    let expects = [
        vec![(Number, "1"), (Lt, "<"), (Number, "2"), (EOF, "\0")],
        vec![(Number, "1"), (Gt, ">"), (Number, "2"), (EOF, "\0")],
        vec![(Number, "1"), (Eq, "=="), (Number, "2"), (EOF, "\0")],
        vec![(Number, "1"), (Ne, "!="), (Number, "2"), (EOF, "\0")],
        vec![(Number, "1"), (Le, "<="), (Number, "2"), (EOF, "\0")],
        vec![(Number, "1"), (Ge, ">="), (Number, "2"), (EOF, "\0")],
    ];

    loop_assert(inputs, expects);
}

#[test]
fn lexer_return_test() {
    let inputs = ["return 10;", "return 10 + 20;"];
    let expects = [
        vec![
            (Return, "return"),
            (Number, "10"),
            (SemiColon, ";"),
            (EOF, "\0"),
        ],
        vec![
            (Return, "return"),
            (Number, "10"),
            (Plus, "+"),
            (Number, "20"),
            (SemiColon, ";"),
            (EOF, "\0"),
        ],
    ];

    loop_assert(inputs, expects);
}

#[test]
fn lexer_unary_test() {
    let inputs = ["!5", "-15"];
    let expects = [
        vec![(Bang, "!"), (Number, "5"), (EOF, "\0")],
        vec![(Minus, "-"), (Number, "15"), (EOF, "\0")],
    ];

    loop_assert(inputs, expects);
}
