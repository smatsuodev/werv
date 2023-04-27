use super::{error::ParserError, Parser};
use wervc_ast::{
    ty::{Type, TypeKind},
    *,
};
use wervc_lexer::token::TokenKind;

fn loop_assert<T, U, const N: usize>(inputs: [T; N], expects: [U; N], f: impl Fn(&mut Parser, U))
where
    T: ToString,
{
    for (input, expect) in inputs.into_iter().zip(expects) {
        let mut parser = Parser::new(input);

        f(&mut parser, expect);
    }
}

#[test]
fn parse_error_test() {
    let inputs = ["{ 123", "10 10;", "let x: = 10"];
    let expects = [
        ParserError::UnexpectedToken {
            expected: TokenKind::RBrace,
            actual: TokenKind::EOF,
        },
        ParserError::RequiredSemiColon,
        ParserError::UnexpectedToken {
            expected: TokenKind::Ident,
            actual: TokenKind::Assign,
        },
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(parser.parse_program(), Err(expect));
    })
}

#[test]
fn parse_stmt_test() {
    let inputs = [
        "1 + 2;",
        "1 + 2",
        "let x: int = 1 + 2;",
        "let x: int = 1 + 2",
        "{ 10 };",
        "{ 10 }",
        "x = 1 + 2;",
        "x = 1 + 2",
    ];
    let expects = [
        Statement::ExprStmt(Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Add,
            lhs: Box::new(Expression::Integer(Integer { value: 1 })),
            rhs: Box::new(Expression::Integer(Integer { value: 2 })),
        })),
        Statement::ExprReturnStmt(Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Add,
            lhs: Box::new(Expression::Integer(Integer { value: 1 })),
            rhs: Box::new(Expression::Integer(Integer { value: 2 })),
        })),
        Statement::ExprStmt(Expression::LetExpr(LetExpr {
            name: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            value: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::Integer(Integer { value: 1 })),
                rhs: Box::new(Expression::Integer(Integer { value: 2 })),
            })),
        })),
        Statement::ExprReturnStmt(Expression::LetExpr(LetExpr {
            name: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            value: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::Integer(Integer { value: 1 })),
                rhs: Box::new(Expression::Integer(Integer { value: 2 })),
            })),
        })),
        Statement::ExprStmt(Expression::BlockExpr(BlockExpr {
            statements: vec![Statement::ExprReturnStmt(Expression::Integer(Integer {
                value: 10,
            }))],
        })),
        Statement::ExprReturnStmt(Expression::BlockExpr(BlockExpr {
            statements: vec![Statement::ExprReturnStmt(Expression::Integer(Integer {
                value: 10,
            }))],
        })),
        Statement::ExprStmt(Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Assign,
            lhs: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            rhs: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::Integer(Integer { value: 1 })),
                rhs: Box::new(Expression::Integer(Integer { value: 2 })),
            })),
        })),
        Statement::ExprReturnStmt(Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Assign,
            lhs: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            rhs: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::Integer(Integer { value: 1 })),
                rhs: Box::new(Expression::Integer(Integer { value: 2 })),
            })),
        })),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![Ident {
            name: "x".to_string(),
            offset: 8,
            ty: Type {
                kind: TypeKind::Int,
                ptr_to: None,
            },
        }];
        assert_eq!(expect, parser.parse_stmt().unwrap())
    });
}

#[test]
fn parse_integer_test() {
    let inputs = ["0", "42"];
    let expects = [
        Expression::Integer(Integer { value: 0 }),
        Expression::Integer(Integer { value: 42 }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_integer().unwrap())
    });
}

#[test]
fn parse_binary_expr_test() {
    let inputs = ["1 + (2 - 3) * 4 / 5", "x + y"];
    let expects = [
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Add,
            lhs: Box::new(Expression::Integer(Integer { value: 1 })),
            rhs: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Div,
                lhs: Box::new(Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Mul,
                    lhs: Box::new(Expression::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Sub,
                        lhs: Box::new(Expression::Integer(Integer { value: 2 })),
                        rhs: Box::new(Expression::Integer(Integer { value: 3 })),
                    })),
                    rhs: Box::new(Expression::Integer(Integer { value: 4 })),
                })),
                rhs: Box::new(Expression::Integer(Integer { value: 5 })),
            })),
        }),
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Add,
            lhs: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            rhs: Box::new(Expression::Ident(Ident {
                name: "y".to_string(),
                offset: 16,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![
            Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            },
            Ident {
                name: "y".to_string(),
                offset: 16,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            },
        ];
        assert_eq!(expect, parser.parse_expr().unwrap())
    });
}

#[test]
fn parse_let_expr() {
    let inputs = [
        "let x: int = 1 + 2",
        "let y: int = 0",
        "let foo_bar: int = 1",
        "let _123: int = 1",
        "let id(x: int) = x",
        "let add(x: int, y: int) = x + y",
        "let zero() = 0",
    ];
    let expects = [
        Expression::LetExpr(LetExpr {
            name: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            value: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::Integer(Integer { value: 1 })),
                rhs: Box::new(Expression::Integer(Integer { value: 2 })),
            })),
        }),
        Expression::LetExpr(LetExpr {
            name: Box::new(Expression::Ident(Ident {
                name: "y".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            value: Box::new(Expression::Integer(Integer { value: 0 })),
        }),
        Expression::LetExpr(LetExpr {
            name: Box::new(Expression::Ident(Ident {
                name: "foo_bar".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            value: Box::new(Expression::Integer(Integer { value: 1 })),
        }),
        Expression::LetExpr(LetExpr {
            name: Box::new(Expression::Ident(Ident {
                name: "_123".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            value: Box::new(Expression::Integer(Integer { value: 1 })),
        }),
        Expression::FunctionDefExpr(FunctionDefExpr {
            name: Box::new(Expression::Ident(Ident {
                name: "id".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Func,
                    ptr_to: None,
                },
            })),
            params: vec![Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 16,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })],
            body: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 16,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
        }),
        Expression::FunctionDefExpr(FunctionDefExpr {
            name: Box::new(Expression::Ident(Ident {
                name: "add".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Func,
                    ptr_to: None,
                },
            })),
            params: vec![
                Expression::Ident(Ident {
                    name: "x".to_string(),
                    offset: 16,
                    ty: Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    },
                }),
                Expression::Ident(Ident {
                    name: "y".to_string(),
                    offset: 24,
                    ty: Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    },
                }),
            ],
            body: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::Ident(Ident {
                    name: "x".to_string(),
                    offset: 16,
                    ty: Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    },
                })),
                rhs: Box::new(Expression::Ident(Ident {
                    name: "y".to_string(),
                    offset: 24,
                    ty: Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    },
                })),
            })),
        }),
        Expression::FunctionDefExpr(FunctionDefExpr {
            name: Box::new(Expression::Ident(Ident {
                name: "zero".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Func,
                    ptr_to: None,
                },
            })),

            params: vec![],
            body: Box::new(Expression::Integer(Integer { value: 0 })),
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_let_expr().unwrap())
    });
}

#[test]
fn parse_block_expr() {
    let inputs = [
        "{ 10 }",
        "{ let x: int = 10; x }",
        "{ let x: int = 10; }",
        "{ let x: int = { 10 } }",
        "{ return 10; }",
    ];
    let expects = [
        Expression::BlockExpr(BlockExpr {
            statements: vec![Statement::ExprReturnStmt(Expression::Integer(Integer {
                value: 10,
            }))],
        }),
        Expression::BlockExpr(BlockExpr {
            statements: vec![
                Statement::ExprStmt(Expression::LetExpr(LetExpr {
                    name: Box::new(Expression::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type {
                            kind: TypeKind::Int,
                            ptr_to: None,
                        },
                    })),
                    value: Box::new(Expression::Integer(Integer { value: 10 })),
                })),
                Statement::ExprReturnStmt(Expression::Ident(Ident {
                    name: "x".to_string(),
                    offset: 8,
                    ty: Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    },
                })),
            ],
        }),
        Expression::BlockExpr(BlockExpr {
            statements: vec![Statement::ExprStmt(Expression::LetExpr(LetExpr {
                name: Box::new(Expression::Ident(Ident {
                    name: "x".to_string(),
                    offset: 8,
                    ty: Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    },
                })),
                value: Box::new(Expression::Integer(Integer { value: 10 })),
            }))],
        }),
        Expression::BlockExpr(BlockExpr {
            statements: vec![Statement::ExprReturnStmt(Expression::LetExpr(LetExpr {
                name: Box::new(Expression::Ident(Ident {
                    name: "x".to_string(),
                    offset: 8,
                    ty: Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    },
                })),
                value: Box::new(Expression::BlockExpr(BlockExpr {
                    statements: vec![Statement::ExprReturnStmt(Expression::Integer(Integer {
                        value: 10,
                    }))],
                })),
            }))],
        }),
        Expression::BlockExpr(BlockExpr {
            statements: vec![Statement::ExprStmt(Expression::ReturnExpr(ReturnExpr {
                value: Box::new(Expression::Integer(Integer { value: 10 })),
            }))],
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_block_expr().unwrap())
    });
}

#[test]
fn parse_assign_test() {
    let inputs = ["x = 1 + 2", "x = y", "x = { 10 }"];
    let expects = [
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Assign,
            lhs: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            rhs: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::Integer(Integer { value: 1 })),
                rhs: Box::new(Expression::Integer(Integer { value: 2 })),
            })),
        }),
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Assign,
            lhs: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            rhs: Box::new(Expression::Ident(Ident {
                name: "y".to_string(),
                offset: 16,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
        }),
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Assign,
            lhs: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            rhs: Box::new(Expression::BlockExpr(BlockExpr {
                statements: vec![Statement::ExprReturnStmt(Expression::Integer(Integer {
                    value: 10,
                }))],
            })),
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![
            Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            },
            Ident {
                name: "y".to_string(),
                offset: 16,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            },
        ];
        assert_eq!(expect, parser.parse_assign().unwrap())
    });
}

#[test]
fn parse_call_test() {
    let inputs = ["foo()", "foo(1,2,3)"];
    let expects = [
        Expression::CallExpr(CallExpr {
            func: Box::new(Expression::Ident(Ident {
                name: "foo".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            args: vec![],
        }),
        Expression::CallExpr(CallExpr {
            func: Box::new(Expression::Ident(Ident {
                name: "foo".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
            args: vec![
                Expression::Integer(Integer { value: 1 }),
                Expression::Integer(Integer { value: 2 }),
                Expression::Integer(Integer { value: 3 }),
            ],
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![Ident {
            name: "foo".to_string(),
            offset: 8,
            ty: Type {
                kind: TypeKind::Int,
                ptr_to: None,
            },
        }];
        assert_eq!(expect, parser.parse_call().unwrap())
    });
}

#[test]
fn parse_relation_test() {
    let inputs = ["1 < 2", "1 <= 2", "1 > 2", "1 >= 2", "1 == 2", "1 != 2"];
    let expects = [
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Lt,
            lhs: Box::new(Expression::Integer(Integer { value: 1 })),
            rhs: Box::new(Expression::Integer(Integer { value: 2 })),
        }),
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Le,
            lhs: Box::new(Expression::Integer(Integer { value: 1 })),
            rhs: Box::new(Expression::Integer(Integer { value: 2 })),
        }),
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Gt,
            lhs: Box::new(Expression::Integer(Integer { value: 1 })),
            rhs: Box::new(Expression::Integer(Integer { value: 2 })),
        }),
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Ge,
            lhs: Box::new(Expression::Integer(Integer { value: 1 })),
            rhs: Box::new(Expression::Integer(Integer { value: 2 })),
        }),
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Eq,
            lhs: Box::new(Expression::Integer(Integer { value: 1 })),
            rhs: Box::new(Expression::Integer(Integer { value: 2 })),
        }),
        Expression::BinaryExpr(BinaryExpr {
            kind: BinaryExprKind::Ne,
            lhs: Box::new(Expression::Integer(Integer { value: 1 })),
            rhs: Box::new(Expression::Integer(Integer { value: 2 })),
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_relation().unwrap())
    });
}

#[test]
fn parse_if_expr_test() {
    let inputs = [
        "if 1 < 2 { 10 } else { 20 }",
        "if 1 < 2 { 10 }",
        "if true false",
    ];
    let expects = [
        Expression::IfExpr(IfExpr {
            condition: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Lt,
                lhs: Box::new(Expression::Integer(Integer { value: 1 })),
                rhs: Box::new(Expression::Integer(Integer { value: 2 })),
            })),
            consequence: Box::new(Expression::BlockExpr(BlockExpr {
                statements: vec![Statement::ExprReturnStmt(Expression::Integer(Integer {
                    value: 10,
                }))],
            })),
            alternative: Some(Box::new(Expression::BlockExpr(BlockExpr {
                statements: vec![Statement::ExprReturnStmt(Expression::Integer(Integer {
                    value: 20,
                }))],
            }))),
        }),
        Expression::IfExpr(IfExpr {
            condition: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Lt,
                lhs: Box::new(Expression::Integer(Integer { value: 1 })),
                rhs: Box::new(Expression::Integer(Integer { value: 2 })),
            })),
            consequence: Box::new(Expression::BlockExpr(BlockExpr {
                statements: vec![Statement::ExprReturnStmt(Expression::Integer(Integer {
                    value: 10,
                }))],
            })),
            alternative: None,
        }),
        Expression::IfExpr(IfExpr {
            condition: Box::new(Expression::Boolean(Boolean { value: true })),
            consequence: Box::new(Expression::Boolean(Boolean { value: false })),
            alternative: None,
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_if_expr().unwrap())
    });
}

#[test]
fn parse_bool_test() {
    let inputs = ["true", "false"];
    let expects = [
        Expression::Boolean(Boolean { value: true }),
        Expression::Boolean(Boolean { value: false }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_bool().unwrap())
    });
}

#[test]
fn parse_return_expr_test() {
    let inputs = ["return 10", "return true"];
    let expects = [
        Expression::ReturnExpr(ReturnExpr {
            value: Box::new(Expression::Integer(Integer { value: 10 })),
        }),
        Expression::ReturnExpr(ReturnExpr {
            value: Box::new(Expression::Boolean(Boolean { value: true })),
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_return_expr().unwrap())
    });
}

#[test]
fn parse_unary_test() {
    let inputs = [
        "!true", "-10", "!!true", "-(-10)", "&x", "*p", "*&p", "******q", "&&&&&&p",
    ];
    let expects = [
        Expression::UnaryExpr(UnaryExpr {
            kind: UnaryExprKind::Not,
            expr: Box::new(Expression::Boolean(Boolean { value: true })),
        }),
        Expression::UnaryExpr(UnaryExpr {
            kind: UnaryExprKind::Minus,
            expr: Box::new(Expression::Integer(Integer { value: 10 })),
        }),
        Expression::UnaryExpr(UnaryExpr {
            kind: UnaryExprKind::Not,
            expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Not,
                expr: Box::new(Expression::Boolean(Boolean { value: true })),
            })),
        }),
        Expression::UnaryExpr(UnaryExpr {
            kind: UnaryExprKind::Minus,
            expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Minus,
                expr: Box::new(Expression::Integer(Integer { value: 10 })),
            })),
        }),
        Expression::UnaryExpr(UnaryExpr {
            kind: UnaryExprKind::Addr,
            expr: Box::new(Expression::Ident(Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            })),
        }),
        Expression::UnaryExpr(UnaryExpr {
            kind: UnaryExprKind::Deref,
            expr: Box::new(Expression::Ident(Ident {
                name: "p".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Ptr,
                    ptr_to: Some(Box::new(Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    })),
                },
            })),
        }),
        Expression::UnaryExpr(UnaryExpr {
            kind: UnaryExprKind::Deref,
            expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Addr,
                expr: Box::new(Expression::Ident(Ident {
                    name: "p".to_string(),
                    offset: 8,
                    ty: Type {
                        kind: TypeKind::Ptr,
                        ptr_to: Some(Box::new(Type {
                            kind: TypeKind::Int,
                            ptr_to: None,
                        })),
                    },
                })),
            })),
        }),
        Expression::UnaryExpr(UnaryExpr {
            kind: UnaryExprKind::Deref,
            expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Deref,
                expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                    kind: UnaryExprKind::Deref,
                    expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                        kind: UnaryExprKind::Deref,
                        expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                            kind: UnaryExprKind::Deref,
                            expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                                kind: UnaryExprKind::Deref,
                                expr: Box::new(Expression::Ident(Ident {
                                    name: "q".to_string(),
                                    offset: 8,
                                    ty: Type {
                                        kind: TypeKind::Ptr,
                                        ptr_to: Some(Box::new(Type {
                                            kind: TypeKind::Ptr,
                                            ptr_to: Some(Box::new(Type {
                                                kind: TypeKind::Ptr,
                                                ptr_to: Some(Box::new(Type {
                                                    kind: TypeKind::Ptr,
                                                    ptr_to: Some(Box::new(Type {
                                                        kind: TypeKind::Ptr,
                                                        ptr_to: Some(Box::new(Type {
                                                            kind: TypeKind::Ptr,
                                                            ptr_to: Some(Box::new(Type {
                                                                kind: TypeKind::Int,
                                                                ptr_to: None,
                                                            })),
                                                        })),
                                                    })),
                                                })),
                                            })),
                                        })),
                                    },
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }),
        Expression::UnaryExpr(UnaryExpr {
            kind: UnaryExprKind::Addr,
            expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Addr,
                expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                    kind: UnaryExprKind::Addr,
                    expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                        kind: UnaryExprKind::Addr,
                        expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                            kind: UnaryExprKind::Addr,
                            expr: Box::new(Expression::UnaryExpr(UnaryExpr {
                                kind: UnaryExprKind::Addr,
                                expr: Box::new(Expression::Ident(Ident {
                                    name: "p".to_string(),
                                    offset: 8,
                                    ty: Type {
                                        kind: TypeKind::Ptr,
                                        ptr_to: Some(Box::new(Type {
                                            kind: TypeKind::Int,
                                            ptr_to: None,
                                        })),
                                    },
                                })),
                            })),
                        })),
                    })),
                })),
            })),
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![
            Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                },
            },
            Ident {
                name: "p".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Ptr,
                    ptr_to: Some(Box::new(Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    })),
                },
            },
            Ident {
                name: "q".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Ptr,
                    ptr_to: Some(Box::new(Type {
                        kind: TypeKind::Ptr,
                        ptr_to: Some(Box::new(Type {
                            kind: TypeKind::Ptr,
                            ptr_to: Some(Box::new(Type {
                                kind: TypeKind::Ptr,
                                ptr_to: Some(Box::new(Type {
                                    kind: TypeKind::Ptr,
                                    ptr_to: Some(Box::new(Type {
                                        kind: TypeKind::Ptr,
                                        ptr_to: Some(Box::new(Type {
                                            kind: TypeKind::Int,
                                            ptr_to: None,
                                        })),
                                    })),
                                })),
                            })),
                        })),
                    })),
                },
            },
        ];
        assert_eq!(expect, parser.parse_unary().unwrap())
    });
}

#[test]
fn parse_array_test() {
    let inputs = ["[1,2,3]", "[]"];
    let expects = [
        Expression::Array(Array {
            elements: vec![
                Expression::Integer(Integer { value: 1 }),
                Expression::Integer(Integer { value: 2 }),
                Expression::Integer(Integer { value: 3 }),
            ],
        }),
        Expression::Array(Array { elements: vec![] }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_array().unwrap())
    });
}

#[test]
fn parse_index_test() {
    let inputs = ["array[1]", "array[1+2]", "[1,2,3][0]", "[[1]][0][0]"];
    let expects = [
        Expression::IndexExpr(IndexExpr {
            array: Box::new(Expression::Ident(Ident {
                name: "array".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Ptr,
                    ptr_to: Some(Box::new(Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    })),
                },
            })),
            index: Box::new(Expression::Integer(Integer { value: 1 })),
        }),
        Expression::IndexExpr(IndexExpr {
            array: Box::new(Expression::Ident(Ident {
                name: "array".to_string(),
                offset: 8,
                ty: Type {
                    kind: TypeKind::Ptr,
                    ptr_to: Some(Box::new(Type {
                        kind: TypeKind::Int,
                        ptr_to: None,
                    })),
                },
            })),
            index: Box::new(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::Integer(Integer { value: 1 })),
                rhs: Box::new(Expression::Integer(Integer { value: 2 })),
            })),
        }),
        Expression::IndexExpr(IndexExpr {
            array: Box::new(Expression::Array(Array {
                elements: vec![
                    Expression::Integer(Integer { value: 1 }),
                    Expression::Integer(Integer { value: 2 }),
                    Expression::Integer(Integer { value: 3 }),
                ],
            })),
            index: Box::new(Expression::Integer(Integer { value: 0 })),
        }),
        Expression::IndexExpr(IndexExpr {
            array: Box::new(Expression::IndexExpr(IndexExpr {
                array: Box::new(Expression::Array(Array {
                    elements: vec![Expression::Array(Array {
                        elements: vec![Expression::Integer(Integer { value: 1 })],
                    })],
                })),
                index: Box::new(Expression::Integer(Integer { value: 0 })),
            })),
            index: Box::new(Expression::Integer(Integer { value: 0 })),
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![Ident {
            name: "array".to_string(),
            offset: 8,
            ty: Type {
                kind: TypeKind::Ptr,
                ptr_to: Some(Box::new(Type {
                    kind: TypeKind::Int,
                    ptr_to: None,
                })),
            },
        }];
        assert_eq!(expect, parser.parse_index().unwrap())
    });
}
