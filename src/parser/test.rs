use super::Parser;
use crate::{
    ast::{BinaryExprKind::*, Expression::*, Node, Statement::*, UnaryExprKind::*},
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
fn fib(n) = add(fib(n-1), fib(n-2));
print(fib(10)+10);
fn f(x, y) = {
    let nx = x + 1;
    let ny = y + 1;

    return nx + ny;
};
if a%2 { a } else { 0 };
if false { true } else { !false };
20 - -10;
1==1;
1!=1;
1<1;
1>1;
1<=1;
1>=1;
"input123";
"#];
    let expect = [Node::Program(vec![
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
        FunctionDefStatement {
            name: Ident("fib".into()),
            params: vec![Ident("n".into())],
            body: CallExpr {
                name: Box::new(Ident("add".into())),
                args: vec![
                    CallExpr {
                        name: Box::new(Ident("fib".into())),
                        args: vec![BinaryExpr {
                            kind: Sub,
                            lhs: Box::new(Ident("n".into())),
                            rhs: Box::new(Integer(1)),
                        }],
                    },
                    CallExpr {
                        name: Box::new(Ident("fib".into())),
                        args: vec![BinaryExpr {
                            kind: Sub,
                            lhs: Box::new(Ident("n".into())),
                            rhs: Box::new(Integer(2)),
                        }],
                    },
                ],
            },
        },
        ExprStatement(CallExpr {
            name: Box::new(Ident("print".into())),
            args: vec![BinaryExpr {
                kind: Add,
                lhs: Box::new(CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![Integer(10)],
                }),
                rhs: Box::new(Integer(10)),
            }],
        }),
        FunctionDefStatement {
            name: Ident("f".into()),
            params: vec![Ident("x".into()), Ident("y".into())],
            body: BlockExpr(vec![
                LetStatement {
                    name: Ident("nx".into()),
                    value: BinaryExpr {
                        kind: Add,
                        lhs: Box::new(Ident("x".into())),
                        rhs: Box::new(Integer(1)),
                    },
                },
                LetStatement {
                    name: Ident("ny".into()),
                    value: BinaryExpr {
                        kind: Add,
                        lhs: Box::new(Ident("y".into())),
                        rhs: Box::new(Integer(1)),
                    },
                },
                ReturnStatement(BinaryExpr {
                    kind: Add,
                    lhs: Box::new(Ident("nx".into())),
                    rhs: Box::new(Ident("ny".into())),
                }),
            ]),
        },
        ExprStatement(IfExpr {
            condition: Box::new(BinaryExpr {
                kind: Mod,
                lhs: Box::new(Ident("a".into())),
                rhs: Box::new(Integer(2)),
            }),
            consequence: Box::new(BlockExpr(vec![BlockReturnStatement(Ident("a".into()))])),
            alternative: Some(Box::new(BlockExpr(vec![BlockReturnStatement(Integer(0))]))),
        }),
        ExprStatement(IfExpr {
            condition: Box::new(Boolean(false)),
            consequence: Box::new(BlockExpr(vec![BlockReturnStatement(Boolean(true))])),
            alternative: Some(Box::new(BlockExpr(vec![BlockReturnStatement(UnaryExpr {
                kind: Not,
                expr: Box::new(Boolean(false)),
            })]))),
        }),
        ExprStatement(BinaryExpr {
            kind: Sub,
            lhs: Box::new(Integer(20)),
            rhs: Box::new(UnaryExpr {
                kind: Minus,
                expr: Box::new(Integer(10)),
            }),
        }),
        ExprStatement(BinaryExpr {
            kind: Eq,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(1)),
        }),
        ExprStatement(BinaryExpr {
            kind: Ne,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(1)),
        }),
        ExprStatement(BinaryExpr {
            kind: Lt,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(1)),
        }),
        ExprStatement(BinaryExpr {
            kind: Gt,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(1)),
        }),
        ExprStatement(BinaryExpr {
            kind: Le,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(1)),
        }),
        ExprStatement(BinaryExpr {
            kind: Ge,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(1)),
        }),
        ExprStatement(Str("input123".into())),
    ])];

    loop_test(input, expect, |p| p.parse().unwrap());
}

#[test]
fn parse_return_stmt_test() {
    let input = ["return nx + ny;"];
    let expect = [ReturnStatement(BinaryExpr {
        kind: Add,
        lhs: Box::new(Ident("nx".into())),
        rhs: Box::new(Ident("ny".into())),
    })];

    loop_test(input, expect, |p| p.parse_return_statement().unwrap());
}

#[test]
fn parse_fn_stmt_test() {
    let input = [
        "fn answer() = 42;",
        "fn calc(a, b, c, d, e) = (a + b - c) * d / e;",
        "fn fib(n) = add(fib(n-1), fib(n-2));",
        "fn f(x, y) = { let nx = x + 1; let ny = y + 1; return nx + ny; };",
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
        FunctionDefStatement {
            name: Ident("fib".into()),
            params: vec![Ident("n".into())],
            body: CallExpr {
                name: Box::new(Ident("add".into())),
                args: vec![
                    CallExpr {
                        name: Box::new(Ident("fib".into())),
                        args: vec![BinaryExpr {
                            kind: Sub,
                            lhs: Box::new(Ident("n".into())),
                            rhs: Box::new(Integer(1)),
                        }],
                    },
                    CallExpr {
                        name: Box::new(Ident("fib".into())),
                        args: vec![BinaryExpr {
                            kind: Sub,
                            lhs: Box::new(Ident("n".into())),
                            rhs: Box::new(Integer(2)),
                        }],
                    },
                ],
            },
        },
        FunctionDefStatement {
            name: Ident("f".into()),
            params: vec![Ident("x".into()), Ident("y".into())],
            body: BlockExpr(vec![
                LetStatement {
                    name: Ident("nx".into()),
                    value: BinaryExpr {
                        kind: Add,
                        lhs: Box::new(Ident("x".into())),
                        rhs: Box::new(Integer(1)),
                    },
                },
                LetStatement {
                    name: Ident("ny".into()),
                    value: BinaryExpr {
                        kind: Add,
                        lhs: Box::new(Ident("y".into())),
                        rhs: Box::new(Integer(1)),
                    },
                },
                ReturnStatement(BinaryExpr {
                    kind: Add,
                    lhs: Box::new(Ident("nx".into())),
                    rhs: Box::new(Ident("ny".into())),
                }),
            ]),
        },
    ];

    loop_test(input, expect, |p| p.parse_fn_statement().unwrap());
}

#[test]
fn parse_params_test() {
    let input = ["()", "(a, b, c, d, e)", "(n)", "(x, y)"];
    let expect = [
        vec![],
        vec![
            Ident("a".into()),
            Ident("b".into()),
            Ident("c".into()),
            Ident("d".into()),
            Ident("e".into()),
        ],
        vec![Ident("n".into())],
        vec![Ident("x".into()), Ident("y".into())],
    ];

    loop_test(input, expect, |p| p.parse_params().unwrap());
}

#[test]
fn parse_let_stmt_test() {
    let input = [
        "let a = 1234567890;",
        "let _ = 1 + (2 - 3) * 4 / 5;",
        "let nx = x + 1;",
        "let ny = y + 1;",
    ];
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
        LetStatement {
            name: Ident("nx".to_string()),
            value: BinaryExpr {
                kind: Add,
                lhs: Box::new(Ident("x".to_string())),
                rhs: Box::new(Integer(1)),
            },
        },
        LetStatement {
            name: Ident("ny".to_string()),
            value: BinaryExpr {
                kind: Add,
                lhs: Box::new(Ident("y".to_string())),
                rhs: Box::new(Integer(1)),
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
        "add(fib(n-1), fib(n-2))",
        "print(fib(10)+10)",
        "{ let nx = x + 1; let ny = y + 1; return nx + ny; }",
        "if a%2 { a } else { 0 }",
        "if false { true } else { false }",
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
        CallExpr {
            name: Box::new(Ident("add".into())),
            args: vec![
                CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![BinaryExpr {
                        kind: Sub,
                        lhs: Box::new(Ident("n".into())),
                        rhs: Box::new(Integer(1)),
                    }],
                },
                CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![BinaryExpr {
                        kind: Sub,
                        lhs: Box::new(Ident("n".into())),
                        rhs: Box::new(Integer(2)),
                    }],
                },
            ],
        },
        CallExpr {
            name: Box::new(Ident("print".into())),
            args: vec![BinaryExpr {
                kind: Add,
                lhs: Box::new(CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![Integer(10)],
                }),
                rhs: Box::new(Integer(10)),
            }],
        },
        BlockExpr(vec![
            LetStatement {
                name: Ident("nx".into()),
                value: BinaryExpr {
                    kind: Add,
                    lhs: Box::new(Ident("x".into())),
                    rhs: Box::new(Integer(1)),
                },
            },
            LetStatement {
                name: Ident("ny".into()),
                value: BinaryExpr {
                    kind: Add,
                    lhs: Box::new(Ident("y".into())),
                    rhs: Box::new(Integer(1)),
                },
            },
            ReturnStatement(BinaryExpr {
                kind: Add,
                lhs: Box::new(Ident("nx".into())),
                rhs: Box::new(Ident("ny".into())),
            }),
        ]),
        IfExpr {
            condition: Box::new(BinaryExpr {
                kind: Mod,
                lhs: Box::new(Ident("a".into())),
                rhs: Box::new(Integer(2)),
            }),
            consequence: Box::new(BlockExpr(vec![BlockReturnStatement(Ident("a".into()))])),
            alternative: Some(Box::new(BlockExpr(vec![BlockReturnStatement(Integer(0))]))),
        },
        IfExpr {
            condition: Box::new(Boolean(false)),
            consequence: Box::new(BlockExpr(vec![BlockReturnStatement(Boolean(true))])),
            alternative: Some(Box::new(BlockExpr(vec![BlockReturnStatement(Boolean(
                false,
            ))]))),
        },
    ];

    loop_test(input, expect, |p| p.parse_expression().unwrap());
}

#[test]
fn parse_if_test() {
    let input = ["if a%2 { a } else { 0 };"];
    let expect = [IfExpr {
        condition: Box::new(BinaryExpr {
            kind: Mod,
            lhs: Box::new(Ident("a".into())),
            rhs: Box::new(Integer(2)),
        }),
        consequence: Box::new(BlockExpr(vec![BlockReturnStatement(Ident("a".into()))])),
        alternative: Some(Box::new(BlockExpr(vec![BlockReturnStatement(Integer(0))]))),
    }];

    loop_test(input, expect, |p| p.parse_if().unwrap());
}

#[test]
fn parse_block_test() {
    let input = ["{ let nx = x + 1; let ny = y + 1; return nx + ny; }"];
    let expect = [BlockExpr(vec![
        LetStatement {
            name: Ident("nx".into()),
            value: BinaryExpr {
                kind: Add,
                lhs: Box::new(Ident("x".into())),
                rhs: Box::new(Integer(1)),
            },
        },
        LetStatement {
            name: Ident("ny".into()),
            value: BinaryExpr {
                kind: Add,
                lhs: Box::new(Ident("y".into())),
                rhs: Box::new(Integer(1)),
            },
        },
        ReturnStatement(BinaryExpr {
            kind: Add,
            lhs: Box::new(Ident("nx".into())),
            rhs: Box::new(Ident("ny".into())),
        }),
    ])];

    loop_test(input, expect, |p| p.parse_block().unwrap());
}

#[test]
fn parse_bool_test() {
    let input = ["true", "false"];
    let expect = [Boolean(true), Boolean(false)];

    loop_test(input, expect, |p| p.parse_bool().unwrap());
}

#[test]
fn parse_bool_unary_test() {
    let input = ["!true", "!false"];
    let expect = [
        UnaryExpr {
            kind: Not,
            expr: Box::new(Boolean(true)),
        },
        UnaryExpr {
            kind: Not,
            expr: Box::new(Boolean(false)),
        },
    ];

    loop_test(input, expect, |p| p.parse_bool_unary().unwrap());
}

#[test]
fn parse_bool_primary_test() {
    let input = ["true", "false"];
    let expect = [Boolean(true), Boolean(false)];

    loop_test(input, expect, |p| p.parse_bool_primary().unwrap());
}

#[test]
fn parse_add_test() {
    let input = [
        "1234567890",
        "1 + (2 - 3) * 4 / 5",
        "42",
        "(a + b - c) * d / e",
        "n-1",
        "n-2",
        "add(fib(n-1), fib(n-2))",
        "print(fib(10)+10)",
        "fib(10)+10",
        "x + 1",
        "y + 1",
        "nx + ny",
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
        BinaryExpr {
            kind: Sub,
            lhs: Box::new(Ident("n".into())),
            rhs: Box::new(Integer(1)),
        },
        BinaryExpr {
            kind: Sub,
            lhs: Box::new(Ident("n".into())),
            rhs: Box::new(Integer(2)),
        },
        CallExpr {
            name: Box::new(Ident("add".into())),
            args: vec![
                CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![BinaryExpr {
                        kind: Sub,
                        lhs: Box::new(Ident("n".into())),
                        rhs: Box::new(Integer(1)),
                    }],
                },
                CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![BinaryExpr {
                        kind: Sub,
                        lhs: Box::new(Ident("n".into())),
                        rhs: Box::new(Integer(2)),
                    }],
                },
            ],
        },
        CallExpr {
            name: Box::new(Ident("print".into())),
            args: vec![BinaryExpr {
                kind: Add,
                lhs: Box::new(CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![Integer(10)],
                }),
                rhs: Box::new(Integer(10)),
            }],
        },
        BinaryExpr {
            kind: Add,
            lhs: Box::new(CallExpr {
                name: Box::new(Ident("fib".into())),
                args: vec![Integer(10)],
            }),
            rhs: Box::new(Integer(10)),
        },
        BinaryExpr {
            kind: Add,
            lhs: Box::new(Ident("x".into())),
            rhs: Box::new(Integer(1)),
        },
        BinaryExpr {
            kind: Add,
            lhs: Box::new(Ident("y".into())),
            rhs: Box::new(Integer(1)),
        },
        BinaryExpr {
            kind: Add,
            lhs: Box::new(Ident("nx".into())),
            rhs: Box::new(Ident("ny".into())),
        },
    ];

    loop_test(input, expect, |p| p.parse_add().unwrap());
}

#[test]
fn parse_mul_test() {
    let input = [
        "1234567890",
        "(2 - 3) * 4 / 5",
        "42",
        "(a + b - c) * d / e",
        "print(fib(10)+10)",
        "fib(10)",
    ];
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
        CallExpr {
            name: Box::new(Ident("print".into())),
            args: vec![BinaryExpr {
                kind: Add,
                lhs: Box::new(CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![Integer(10)],
                }),
                rhs: Box::new(Integer(10)),
            }],
        },
        CallExpr {
            name: Box::new(Ident("fib".into())),
            args: vec![Integer(10)],
        },
    ];

    loop_test(input, expect, |p| p.parse_mul().unwrap());
}

#[test]

fn parse_unary_test() {
    let input = ["-20", "-2"];
    let expect = [
        UnaryExpr {
            kind: Minus,
            expr: Box::new(Integer(20)),
        },
        UnaryExpr {
            kind: Minus,
            expr: Box::new(Integer(2)),
        },
    ];

    loop_test(input, expect, |p| p.parse_unary().unwrap());
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
        "add(fib(n-1), fib(n-2))",
        "print(fib(10)+10)",
        "x",
        "y",
        "nx",
        "ny",
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
        CallExpr {
            name: Box::new(Ident("add".into())),
            args: vec![
                CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![BinaryExpr {
                        kind: Sub,
                        lhs: Box::new(Ident("n".into())),
                        rhs: Box::new(Integer(1)),
                    }],
                },
                CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![BinaryExpr {
                        kind: Sub,
                        lhs: Box::new(Ident("n".into())),
                        rhs: Box::new(Integer(2)),
                    }],
                },
            ],
        },
        CallExpr {
            name: Box::new(Ident("print".into())),
            args: vec![BinaryExpr {
                kind: Add,
                lhs: Box::new(CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![Integer(10)],
                }),
                rhs: Box::new(Integer(10)),
            }],
        },
        Ident("x".into()),
        Ident("y".into()),
        Ident("nx".into()),
        Ident("ny".into()),
    ];

    loop_test(input, expect, |p| p.parse_primary().unwrap());
}

#[test]
fn parse_call_expression_test() {
    let input = ["add(fib(n-1), fib(n-2))", "print(fib(10)+10)"];
    let expect = [
        CallExpr {
            name: Box::new(Ident("add".into())),
            args: vec![
                CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![BinaryExpr {
                        kind: Sub,
                        lhs: Box::new(Ident("n".into())),
                        rhs: Box::new(Integer(1)),
                    }],
                },
                CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![BinaryExpr {
                        kind: Sub,
                        lhs: Box::new(Ident("n".into())),
                        rhs: Box::new(Integer(2)),
                    }],
                },
            ],
        },
        CallExpr {
            name: Box::new(Ident("print".into())),
            args: vec![BinaryExpr {
                kind: Add,
                lhs: Box::new(CallExpr {
                    name: Box::new(Ident("fib".into())),
                    args: vec![Integer(10)],
                }),
                rhs: Box::new(Integer(10)),
            }],
        },
    ];

    loop_test(input, expect, |p| p.parse_call_expression().unwrap());
}

#[test]
fn parse_args_test() {
    let input = ["(fib(n-1), fib(n-2))", "(fib(10)+10)", "(10)"];
    let expect = [
        vec![
            CallExpr {
                name: Box::new(Ident("fib".into())),
                args: vec![BinaryExpr {
                    kind: Sub,
                    lhs: Box::new(Ident("n".into())),
                    rhs: Box::new(Integer(1)),
                }],
            },
            CallExpr {
                name: Box::new(Ident("fib".into())),
                args: vec![BinaryExpr {
                    kind: Sub,
                    lhs: Box::new(Ident("n".into())),
                    rhs: Box::new(Integer(2)),
                }],
            },
        ],
        vec![BinaryExpr {
            kind: Add,
            lhs: Box::new(CallExpr {
                name: Box::new(Ident("fib".into())),
                args: vec![Integer(10)],
            }),
            rhs: Box::new(Integer(10)),
        }],
        vec![Integer(10)],
    ];

    loop_test(input, expect, |p| p.parse_args().unwrap());
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
    let input = ["1234567890", "1", "2", "3", "4", "5", "42", "10"];
    let expect = [
        Integer(1234567890),
        Integer(1),
        Integer(2),
        Integer(3),
        Integer(4),
        Integer(5),
        Integer(42),
        Integer(10),
    ];

    loop_test(input, expect, |p| p.parse_integer().unwrap());
}

#[test]
fn parse_ident_test() {
    let input = [
        "a", "_", "answer", "calc", "b", "c", "d", "e", "add", "fib", "n", "print", "f", "x", "y",
        "nx", "ny",
    ];
    let expect = [
        Ident("a".into()),
        Ident("_".into()),
        Ident("answer".into()),
        Ident("calc".into()),
        Ident("b".into()),
        Ident("c".into()),
        Ident("d".into()),
        Ident("e".into()),
        Ident("add".into()),
        Ident("fib".into()),
        Ident("n".into()),
        Ident("print".into()),
        Ident("f".into()),
        Ident("x".into()),
        Ident("y".into()),
        Ident("nx".into()),
        Ident("ny".into()),
    ];

    loop_test(input, expect, |p| p.parse_ident().unwrap());
}
