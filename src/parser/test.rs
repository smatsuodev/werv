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
let a = 1234567890;
let _ = 1 + (2 - 3) * 4 / 5;
fn answer() = 42;
fn calc(a, b, c, d, e) = (a + b - c) * d / e;
"#];
    let expect = [vec![
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
        FunctionDefStatement {
            name: Ident("answer".into()),
            params: vec![],
            body: Integer(42),
        },
        FunctionDefStatement {
            name: Ident("calc".into()),
            params: vec![
                Ident("a".into()),
                Ident("b".into()),
                Ident("c".into()),
                Ident("d".into()),
                Ident("e".into()),
            ],
            body: BinaryExpr {
                kind: Div,
                lhs: Box::new(BinaryExpr {
                    kind: Mul,
                    lhs: Box::new(BinaryExpr {
                        kind: Sub,
                        lhs: Box::new(BinaryExpr {
                            kind: Add,
                            lhs: Box::new(Ident("a".into())),
                            rhs: Box::new(Ident("b".into())),
                        }),
                        rhs: Box::new(Ident("c".into())),
                    }),
                    rhs: Box::new(Ident("d".into())),
                }),
                rhs: Box::new(Ident("e".into())),
            },
        },
    ]];

    loop_test(input, expect, |p| p.parse().unwrap());
}

#[test]
fn parse_fn_stmt() {
    let input = [
        "fn answer() = 42;",
        "fn calc(a, b, c, d, e) = (a + b - c) * d / e;",
    ];
    let expect = [
        FunctionDefStatement {
            name: Ident("answer".into()),
            params: vec![],
            body: Integer(42),
        },
        FunctionDefStatement {
            name: Ident("calc".into()),
            params: vec![
                Ident("a".into()),
                Ident("b".into()),
                Ident("c".into()),
                Ident("d".into()),
                Ident("e".into()),
            ],
            body: BinaryExpr {
                kind: Div,
                lhs: Box::new(BinaryExpr {
                    kind: Mul,
                    lhs: Box::new(BinaryExpr {
                        kind: Sub,
                        lhs: Box::new(BinaryExpr {
                            kind: Add,
                            lhs: Box::new(Ident("a".into())),
                            rhs: Box::new(Ident("b".into())),
                        }),
                        rhs: Box::new(Ident("c".into())),
                    }),
                    rhs: Box::new(Ident("d".into())),
                }),
                rhs: Box::new(Ident("e".into())),
            },
        },
    ];

    loop_test(input, expect, |p| p.parse_fn_statement().unwrap());
}

#[test]
fn parse_params_test() {
    let input = ["()", "(a, b, c, d, e)"];
    let expect = [
        vec![],
        vec![
            Ident("a".into()),
            Ident("b".into()),
            Ident("c".into()),
            Ident("d".into()),
            Ident("e".into()),
        ],
    ];

    loop_test(input, expect, |p| p.parse_params().unwrap());
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
    let input = [
        "1234567890",
        "1 + (2 - 3) * 4 / 5",
        "42",
        "(a + b - c) * d / e",
    ];
    let expect = [
        Integer(1234567890),
        BinaryExpr {
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
        Integer(42),
        BinaryExpr {
            kind: Div,
            lhs: Box::new(BinaryExpr {
                kind: Mul,
                lhs: Box::new(BinaryExpr {
                    kind: Sub,
                    lhs: Box::new(BinaryExpr {
                        kind: Add,
                        lhs: Box::new(Ident("a".into())),
                        rhs: Box::new(Ident("b".into())),
                    }),
                    rhs: Box::new(Ident("c".into())),
                }),
                rhs: Box::new(Ident("d".into())),
            }),
            rhs: Box::new(Ident("e".into())),
        },
    ];

    loop_test(input, expect, |p| p.parse_expression().unwrap());
}

#[test]
fn parse_add_test() {
    let input = [
        "1234567890",
        "1 + (2 - 3) * 4 / 5",
        "42",
        "(a + b - c) * d / e",
    ];
    let expect = [
        Integer(1234567890),
        BinaryExpr {
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
        Integer(42),
        BinaryExpr {
            kind: Div,
            lhs: Box::new(BinaryExpr {
                kind: Mul,
                lhs: Box::new(BinaryExpr {
                    kind: Sub,
                    lhs: Box::new(BinaryExpr {
                        kind: Add,
                        lhs: Box::new(Ident("a".into())),
                        rhs: Box::new(Ident("b".into())),
                    }),
                    rhs: Box::new(Ident("c".into())),
                }),
                rhs: Box::new(Ident("d".into())),
            }),
            rhs: Box::new(Ident("e".into())),
        },
    ];

    loop_test(input, expect, |p| p.parse_add().unwrap());
}

#[test]
fn parse_mul_test() {
    let input = ["1234567890", "(2 - 3) * 4 / 5", "42", "(a + b - c) * d / e"];
    let expect = [
        Integer(1234567890),
        BinaryExpr {
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
        },
        Integer(42),
        BinaryExpr {
            kind: Div,
            lhs: Box::new(BinaryExpr {
                kind: Mul,
                lhs: Box::new(BinaryExpr {
                    kind: Sub,
                    lhs: Box::new(BinaryExpr {
                        kind: Add,
                        lhs: Box::new(Ident("a".into())),
                        rhs: Box::new(Ident("b".into())),
                    }),
                    rhs: Box::new(Ident("c".into())),
                }),
                rhs: Box::new(Ident("d".into())),
            }),
            rhs: Box::new(Ident("e".into())),
        },
    ];

    loop_test(input, expect, |p| p.parse_mul().unwrap());
}

#[test]
fn parse_primary_test() {
    let input = [
        "1234567890",
        "1",
        "(2 - 3)",
        "4",
        "5",
        "42",
        "(a + b - c)",
        "d",
        "e",
    ];
    let expect = [
        Integer(1234567890),
        Integer(1),
        BinaryExpr {
            kind: Sub,
            lhs: Box::new(Integer(2)),
            rhs: Box::new(Integer(3)),
        },
        Integer(4),
        Integer(5),
        Integer(42),
        BinaryExpr {
            kind: Sub,
            lhs: Box::new(BinaryExpr {
                kind: Add,
                lhs: Box::new(Ident("a".into())),
                rhs: Box::new(Ident("b".into())),
            }),
            rhs: Box::new(Ident("c".into())),
        },
        Ident("d".into()),
        Ident("e".into()),
    ];

    loop_test(input, expect, |p| p.parse_primary().unwrap());
}

#[test]
fn parse_paren_expr_test() {
    let input = ["(2 - 3)", "(a + b - c)"];
    let expect = [
        BinaryExpr {
            kind: Sub,
            lhs: Box::new(Integer(2)),
            rhs: Box::new(Integer(3)),
        },
        BinaryExpr {
            kind: Sub,
            lhs: Box::new(BinaryExpr {
                kind: Add,
                lhs: Box::new(Ident("a".into())),
                rhs: Box::new(Ident("b".into())),
            }),
            rhs: Box::new(Ident("c".into())),
        },
    ];

    loop_test(input, expect, |p| p.parse_paren_expr().unwrap());
}

#[test]
fn parse_integer_test() {
    let input = ["1234567890", "1", "2", "3", "4", "5", "42"];
    let expect = [
        Integer(1234567890),
        Integer(1),
        Integer(2),
        Integer(3),
        Integer(4),
        Integer(5),
        Integer(42),
    ];

    loop_test(input, expect, |p| p.parse_integer().unwrap());
}

#[test]
fn parse_ident_test() {
    let input = ["a", "_", "answer", "calc", "b", "c", "d", "e"];
    let expect = [
        Ident("a".into()),
        Ident("_".into()),
        Ident("answer".into()),
        Ident("calc".into()),
        Ident("b".into()),
        Ident("c".into()),
        Ident("d".into()),
        Ident("e".into()),
    ];

    loop_test(input, expect, |p| p.parse_ident().unwrap());
}
