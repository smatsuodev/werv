use super::{
    cursor::Cursor,
    token::{
        Token,
        TokenKind::{self, *},
    },
};

fn assert_inputs<const N: usize>(inputs: [&str; N], mut test: impl FnMut(&mut Cursor, &str) -> ()) {
    for input in inputs {
        let mut cursor = Cursor::new(input);

        test(&mut cursor, input);
    }
}

fn assert_tokens<const N: usize>(input: &str, expects: [(TokenKind, u32); N]) {
    let mut cursor = Cursor::new(input);

    for (kind, len) in expects {
        assert_eq!(cursor.advance_token(), Token::new(kind, len));
    }

    assert_eq!(cursor.advance_token(), Token::new(Eof, 0));
}

#[test]
fn test_unsigned_number() {
    let inputs = ["0", "101"];

    assert_inputs(inputs, |cursor, input| {
        assert_eq!(
            cursor.advance_token(),
            Token::new(Number, input.len().try_into().unwrap())
        );
    });
}

#[test]
fn test_signed_number() {
    let inputs = ["-0", "-101"];

    assert_inputs(inputs, |cursor, input| {
        assert_eq!(cursor.advance_token(), Token::new(Minus, 1));
        assert_eq!(
            cursor.advance_token(),
            Token::new(Number, input[1..].len().try_into().unwrap())
        );
    });
}

#[test]
fn test_arithmetic() {
    let input = "0 + 11 - 222 * 3333 / 44444";
    let expects = [
        (Number, 1),
        (Plus, 1),
        (Number, 2),
        (Minus, 1),
        (Number, 3),
        (Asterisk, 1),
        (Number, 4),
        (Slash, 1),
        (Number, 5),
    ];

    assert_tokens(input, expects);
}

#[test]
fn test_condition() {
    let tests = [
        ("10==10", Eq),
        ("10!=10", Ne),
        ("10<10", Lt),
        ("10<=10", Le),
        ("10>10", Gt),
        ("10>=10", Ge),
    ];

    for (input, kind) in tests {
        let mut cursor = Cursor::new(input);

        assert_eq!(cursor.advance_token(), Token::new(Number, 2));
        assert_eq!(
            cursor.advance_token(),
            Token::new(kind, (input.len() - 4).try_into().unwrap())
        );
        assert_eq!(cursor.advance_token(), Token::new(Number, 2));
    }
}

#[test]
fn test_parenthesis() {
    let input = "((10+1)*3+1)/20";
    let expects = [
        (LParen, 1),
        (LParen, 1),
        (Number, 2),
        (Plus, 1),
        (Number, 1),
        (RParen, 1),
        (Asterisk, 1),
        (Number, 1),
        (Plus, 1),
        (Number, 1),
        (RParen, 1),
        (Slash, 1),
        (Number, 2),
    ];

    assert_tokens(input, expects);
}
