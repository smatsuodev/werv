use super::Parser;
use crate::{
    ast::{BinaryExprKind::*, Expression::*, Statement::*},
    lexer::Lexer,
};

fn loop_test<T, U, P, const N: usize>(input: [T; N], expect: [U; N], f: impl Fn(&mut Parser) -> P)
where
    T: ToString,
    U: Clone + std::fmt::Debug,
    P: std::cmp::PartialEq<U> + std::fmt::Debug,
{
    for i in 0..N {
        let l = Lexer::new(input[i].to_string());
        let mut p = Parser::new(l);

        assert_eq!(f(&mut p), expect[i].clone());
    }
}

#[test]
fn parse_test() {
    let input = [r#"
let a = 10;
let b = 1 + (2 - 3) * 4 / 5;"#];
    let expect = [vec![
        LetStatement {
            name: Ident("a".into()),
            value: Integer(10),
        },
        LetStatement {
            name: Ident("b".into()),
            value: BinaryExpr {
                kind: Add,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(BinaryExpr {
                    kind: Div,
                    lhs: Box::new(BinaryExpr {
                        kind: Mul,
                        lhs: Box::new(BinaryExpr {
                            kind: Sub,
                            lhs: Box::new(Integer(2)),
                            rhs: Box::new(Integer(3)),
                        }),
                        rhs: Box::new(Integer(4)),
                    }),
                    rhs: Box::new(Integer(5)),
                }),
            },
        },
    ]];

    loop_test(input, expect, |p| p.parse().unwrap());
}

#[test]
fn parse_let_stmt() {
    let input = ["let a = 1234567890;", "let _ = 1 + (2 - 3) * 4 / 5;"];
    let expect = [
        LetStatement {
            name: Ident("a".to_string()),
            value: Integer(1234567890),
        },
        LetStatement {
            name: Ident("_".to_string()),
            value: BinaryExpr {
                kind: Add,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(BinaryExpr {
                    kind: Div,
                    lhs: Box::new(BinaryExpr {
                        kind: Mul,
                        lhs: Box::new(BinaryExpr {
                            kind: Sub,
                            lhs: Box::new(Integer(2)),
                            rhs: Box::new(Integer(3)),
                        }),
                        rhs: Box::new(Integer(4)),
                    }),
                    rhs: Box::new(Integer(5)),
                }),
            },
        },
    ];

    loop_test(input, expect, |p| p.parse_let_statement().unwrap());
}

#[test]
fn parse_expression_test() {
    let input = ["1 + (2 - 3) * 4 / 5"];
    let expect = [BinaryExpr {
        kind: Add,
        lhs: Box::new(Integer(1)),
        rhs: Box::new(BinaryExpr {
            kind: Div,
            lhs: Box::new(BinaryExpr {
                kind: Mul,
                lhs: Box::new(BinaryExpr {
                    kind: Sub,
                    lhs: Box::new(Integer(2)),
                    rhs: Box::new(Integer(3)),
                }),
                rhs: Box::new(Integer(4)),
            }),
            rhs: Box::new(Integer(5)),
        }),
    }];

    loop_test(input, expect, |p| p.parse_expression().unwrap());
}

#[test]
fn parse_add_test() {
    let input = ["1 + (2 - 3) * 4 / 5"];
    let expect = [BinaryExpr {
        kind: Add,
        lhs: Box::new(Integer(1)),
        rhs: Box::new(BinaryExpr {
            kind: Div,
            lhs: Box::new(BinaryExpr {
                kind: Mul,
                lhs: Box::new(BinaryExpr {
                    kind: Sub,
                    lhs: Box::new(Integer(2)),
                    rhs: Box::new(Integer(3)),
                }),
                rhs: Box::new(Integer(4)),
            }),
            rhs: Box::new(Integer(5)),
        }),
    }];

    loop_test(input, expect, |p| p.parse_add().unwrap());
}

#[test]
fn parse_mul_test() {
    let input = ["(2 - 3) * 4 / 5"];
    let expect = [BinaryExpr {
        kind: Div,
        lhs: Box::new(BinaryExpr {
            kind: Mul,
            lhs: Box::new(BinaryExpr {
                kind: Sub,
                lhs: Box::new(Integer(2)),
                rhs: Box::new(Integer(3)),
            }),
            rhs: Box::new(Integer(4)),
        }),
        rhs: Box::new(Integer(5)),
    }];

    loop_test(input, expect, |p| p.parse_mul().unwrap());
}

#[test]
fn parse_primary_test() {
    let input = ["a", "1234567890", "_", "1", "(2 - 3)", "4", "5"];
    let expect = [
        Ident("a".to_string()),
        Integer(1234567890),
        Ident("_".to_string()),
        Integer(1),
        BinaryExpr {
            kind: Sub,
            lhs: Box::new(Integer(2)),
            rhs: Box::new(Integer(3)),
        },
        Integer(4),
        Integer(5),
    ];

    loop_test(input, expect, |p| p.parse_primary().unwrap());
}

#[test]
fn parse_integer_test() {
    let input = ["1234567890", "1", "2", "3", "4", "5"];
    let expect = [
        Integer(1234567890),
        Integer(1),
        Integer(2),
        Integer(3),
        Integer(4),
        Integer(5),
    ];

    loop_test(input, expect, |p| p.parse_integer().unwrap());
}

#[test]
fn parse_ident_test() {
    let input = ["a", "_"];
    let expect = [Ident("a".into()), Ident("_".into())];

    loop_test(input, expect, |p| p.parse_ident().unwrap());
}
