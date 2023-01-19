use crate::token::TokenKind::*;

#[test]
fn lexer_number_test() {
    let inputs = ["0", "42"];

    for input in inputs {
        let mut lexer = Lexer::new(input);

        assert_eq!(Number(input.to_string()), lexer.next_token());
        assert_eq!(EOF, lexer.next_token());
    }
}
