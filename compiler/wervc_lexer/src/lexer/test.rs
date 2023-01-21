use crate::{
    lexer::Lexer,
    token::{Token, TokenKind::*},
};

fn loop_assert<T, const N: usize, const M: usize>(
    inputs: [impl ToString; N],
    expects: [[T; M]; N],
    f: impl Fn(&mut Lexer, [T; M]),
) {
    for (input, expects) in inputs.into_iter().zip(expects) {
        let mut lexer = Lexer::new(input);

        f(&mut lexer, expects);
    }
}

#[test]
fn lexer_number_test() {
    let inputs = ["0", "42"];

    for input in inputs {
        let mut lexer = Lexer::new(input);

        assert_eq!(Token::new(Number, input), lexer.next_token());
        assert_eq!(Token::new(EOF, '\0'), lexer.next_token());
    }
}

#[test]
fn lexer_arithmetic_test() {
    let inputs = ["1 + 2 - 3 * 4 / 5"];
    let expects = [[
        (Number, "1"),
        (Plus, "+"),
        (Number, "2"),
        (Minus, "-"),
        (Number, "3"),
        (Asterisk, "*"),
        (Number, "4"),
        (Slash, "/"),
        (Number, "5"),
        (EOF, "\0"),
    ]];

    loop_assert(inputs, expects, |lexer, expects| {
        for (kind, literal) in expects {
            assert_eq!(Token::new(kind, literal), lexer.next_token());
        }
    });
}
