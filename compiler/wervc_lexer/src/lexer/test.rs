use crate::{
    lexer::Lexer,
    token::TokenKind::{self, *},
};

fn loop_assert<const N: usize, const M: usize>(
    inputs: [impl ToString; N],
    expects: [[TokenKind; M]; N],
) {
    for (input, expects) in inputs.into_iter().zip(expects) {
        let mut lexer = Lexer::new(input);

        for expect in expects {
            assert_eq!(expect, lexer.next_token())
        }
    }
}

#[test]
fn lexer_number_test() {
    let inputs = ["0", "42"];

    for input in inputs {
        let mut lexer = Lexer::new(input);

        assert_eq!(Number(input.to_string()), lexer.next_token());
        assert_eq!(EOF, lexer.next_token());
    }
}

#[test]
fn lexer_arithmetic_test() {
    let inputs = ["1+2-3*4/5"];
    let expects = [[
        Number("1".to_string()),
        Plus,
        Number("2".to_string()),
        Minus,
        Number("3".to_string()),
        Asterisk,
        Number("4".to_string()),
        Slash,
        Number("5".to_string()),
        EOF,
    ]];

    loop_assert(inputs, expects);
}
