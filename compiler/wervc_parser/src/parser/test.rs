use super::{error::ParserError, Parser};
use wervc_ast::{ty::Type, *};
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
        Statement::ExprStmt(Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 2 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        )),
        Statement::ExprReturnStmt(Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 2 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        )),
        Statement::ExprStmt(Expression::new(
            Expr::LetExpr(LetExpr {
                name: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                value: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 1 }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 2 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        )),
        Statement::ExprReturnStmt(Expression::new(
            Expr::LetExpr(LetExpr {
                name: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                value: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 1 }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 2 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        )),
        Statement::ExprStmt(Expression::new(
            Expr::BlockExpr(BlockExpr {
                statements: vec![Statement::ExprReturnStmt(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                ))],
            }),
            Type::int(),
        )),
        Statement::ExprReturnStmt(Expression::new(
            Expr::BlockExpr(BlockExpr {
                statements: vec![Statement::ExprReturnStmt(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                ))],
            }),
            Type::int(),
        )),
        Statement::ExprStmt(Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Assign,
                lhs: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 1 }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 2 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        )),
        Statement::ExprReturnStmt(Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Assign,
                lhs: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 1 }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 2 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        )),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![Ident {
            name: "x".to_string(),
            offset: 8,
            ty: Type::int(),
        }];
        assert_eq!(expect, parser.parse_stmt().unwrap())
    });
}

#[test]
fn parse_integer_test() {
    let inputs = ["0", "42"];
    let expects = [
        Expression::new(Expr::Integer(Integer { value: 0 }), Type::int()),
        Expression::new(Expr::Integer(Integer { value: 42 }), Type::int()),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_integer().unwrap())
    });
}

#[test]
fn parse_binary_expr_test() {
    let inputs = ["1 + (2 - 3) * 4 / 5", "x + y"];
    let expects = [
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Div,
                        lhs: Box::new(Expression::new(
                            Expr::BinaryExpr(BinaryExpr {
                                kind: BinaryExprKind::Mul,
                                lhs: Box::new(Expression::new(
                                    Expr::BinaryExpr(BinaryExpr {
                                        kind: BinaryExprKind::Sub,
                                        lhs: Box::new(Expression::new(
                                            Expr::Integer(Integer { value: 2 }),
                                            Type::int(),
                                        )),
                                        rhs: Box::new(Expression::new(
                                            Expr::Integer(Integer { value: 3 }),
                                            Type::int(),
                                        )),
                                    }),
                                    Type::int(),
                                )),
                                rhs: Box::new(Expression::new(
                                    Expr::Integer(Integer { value: 4 }),
                                    Type::int(),
                                )),
                            }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 5 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "y".to_string(),
                        offset: 16,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![
            Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type::int(),
            },
            Ident {
                name: "y".to_string(),
                offset: 16,
                ty: Type::int(),
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
        Expression::new(
            Expr::LetExpr(LetExpr {
                name: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                value: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 1 }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 2 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::LetExpr(LetExpr {
                name: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "y".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                value: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 0 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::LetExpr(LetExpr {
                name: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "foo_bar".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                value: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::LetExpr(LetExpr {
                name: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "_123".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                value: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::FunctionDefExpr(FunctionDefExpr {
                name: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "id".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                params: vec![Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 16,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )],
                body: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 16,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::FunctionDefExpr(FunctionDefExpr {
                name: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "add".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::func(Box::new(Type::int())),
                )),
                params: vec![
                    Expression::new(
                        Expr::Ident(Ident {
                            name: "x".to_string(),
                            offset: 16,
                            ty: Type::int(),
                        }),
                        Type::int(),
                    ),
                    Expression::new(
                        Expr::Ident(Ident {
                            name: "y".to_string(),
                            offset: 24,
                            ty: Type::int(),
                        }),
                        Type::int(),
                    ),
                ],
                body: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Expression::new(
                            Expr::Ident(Ident {
                                name: "x".to_string(),
                                offset: 16,
                                ty: Type::int(),
                            }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Ident(Ident {
                                name: "y".to_string(),
                                offset: 24,
                                ty: Type::int(),
                            }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::func(Box::new(Type::int())),
        ),
        Expression::new(
            Expr::FunctionDefExpr(FunctionDefExpr {
                name: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "zero".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::func(Box::new(Type::int())),
                )),
                params: vec![],
                body: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 0 }),
                    Type::int(),
                )),
            }),
            Type::func(Box::new(Type::int())),
        ),
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
        Expression::new(
            Expr::BlockExpr(BlockExpr {
                statements: vec![Statement::ExprReturnStmt(Expression::new(
                    Expr::Integer(Integer { value: 10 }),
                    Type::int(),
                ))],
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BlockExpr(BlockExpr {
                statements: vec![
                    Statement::ExprStmt(Expression::new(
                        Expr::LetExpr(LetExpr {
                            name: Box::new(Expression::new(
                                Expr::Ident(Ident {
                                    name: "x".to_string(),
                                    offset: 8,
                                    ty: Type::int(),
                                }),
                                Type::int(),
                            )),
                            value: Box::new(Expression::new(
                                Expr::Integer(Integer { value: 10 }),
                                Type::int(),
                            )),
                        }),
                        Type::int(),
                    )),
                    Statement::ExprReturnStmt(Expression::new(
                        Expr::Ident(Ident {
                            name: "x".to_string(),
                            offset: 8,
                            ty: Type::int(),
                        }),
                        Type::int(),
                    )),
                ],
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BlockExpr(BlockExpr {
                statements: vec![Statement::ExprStmt(Expression::new(
                    Expr::LetExpr(LetExpr {
                        name: Box::new(Expression::new(
                            Expr::Ident(Ident {
                                name: "x".to_string(),
                                offset: 8,
                                ty: Type::int(),
                            }),
                            Type::int(),
                        )),
                        value: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 10 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                ))],
            }),
            Type::nil(),
        ),
        Expression::new(
            Expr::BlockExpr(BlockExpr {
                statements: vec![Statement::ExprStmt(Expression::new(
                    Expr::LetExpr(LetExpr {
                        name: Box::new(Expression::new(
                            Expr::Ident(Ident {
                                name: "x".to_string(),
                                offset: 8,
                                ty: Type::int(),
                            }),
                            Type::int(),
                        )),
                        value: Box::new(Expression::new(
                            Expr::BlockExpr(BlockExpr {
                                statements: vec![Statement::ExprReturnStmt(Expression::new(
                                    Expr::Integer(Integer { value: 10 }),
                                    Type::int(),
                                ))],
                            }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                ))],
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BlockExpr(BlockExpr {
                statements: vec![Statement::ExprStmt(
                    Expression::new(
                        Expr::ReturnExpr(ReturnExpr {
                            value: Box::new(Expression::new(
                                Expr::Integer(Integer { value: 10 }),
                                Type::int(),
                            )),
                        }),
                        Type::int(),
                    )
                    .into(),
                )],
            }),
            Type::int(),
        ),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_block_expr().unwrap())
    });
}

#[test]
fn parse_assign_test() {
    let inputs = ["x = 1 + 2", "x = y", "x = { 10 }"];
    let expects = [
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Assign,
                lhs: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 1 }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 2 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Assign,
                lhs: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "y".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Assign,
                lhs: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::BlockExpr(BlockExpr {
                        statements: vec![Statement::ExprReturnStmt(Expression::new(
                            Expr::Integer(Integer { value: 10 }),
                            Type::int(),
                        ))],
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
    ];
    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![
            Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type::int(),
            },
            Ident {
                name: "y".to_string(),
                offset: 16,
                ty: Type::int(),
            },
        ];
        assert_eq!(expect, parser.parse_assign().unwrap())
    });
}

#[test]
fn parse_call_test() {
    let inputs = ["foo()", "foo(1,2,3)"];
    let expects = [
        Expression::new(
            Expr::CallExpr(CallExpr {
                func: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "foo".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                args: vec![],
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::CallExpr(CallExpr {
                func: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "foo".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
                args: vec![
                    Expression::new(Expr::Integer(Integer { value: 1 }), Type::int()),
                    Expression::new(Expr::Integer(Integer { value: 2 }), Type::int()),
                    Expression::new(Expr::Integer(Integer { value: 3 }), Type::int()),
                ],
            }),
            Type::int(),
        ),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![Ident {
            name: "foo".to_string(),
            offset: 8,
            ty: Type::func(Box::new(Type::int())),
        }];
        assert_eq!(expect, parser.parse_call().unwrap())
    });
}

#[test]
fn parse_relation_test() {
    let inputs = ["1 < 2", "1 <= 2", "1 > 2", "1 >= 2", "1 == 2", "1 != 2"];
    let expects = [
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Lt,
                lhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 2 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Le,
                lhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 2 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Gt,
                lhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 2 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Ge,
                lhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 2 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Eq,
                lhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 2 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Ne,
                lhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
                rhs: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 2 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
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
        Expression::new(
            Expr::IfExpr(IfExpr {
                condition: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Lt,
                        lhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 1 }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 2 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
                consequence: Box::new(Expression::new(
                    Expr::BlockExpr(BlockExpr {
                        statements: vec![Statement::ExprReturnStmt(Expression::new(
                            Expr::Integer(Integer { value: 10 }),
                            Type::int(),
                        ))],
                    }),
                    Type::int(),
                )),
                alternative: Some(Box::new(Expression::new(
                    Expr::BlockExpr(BlockExpr {
                        statements: vec![Statement::ExprReturnStmt(Expression::new(
                            Expr::Integer(Integer { value: 20 }),
                            Type::int(),
                        ))],
                    }),
                    Type::int(),
                ))),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::IfExpr(IfExpr {
                condition: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Lt,
                        lhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 1 }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 2 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
                consequence: Box::new(Expression::new(
                    Expr::BlockExpr(BlockExpr {
                        statements: vec![Statement::ExprReturnStmt(Expression::new(
                            Expr::Integer(Integer { value: 10 }),
                            Type::int(),
                        ))],
                    }),
                    Type::int(),
                )),
                alternative: None,
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::IfExpr(IfExpr {
                condition: Box::new(Expression::new(
                    Expr::Boolean(Boolean { value: true }),
                    Type::int(),
                )),
                consequence: Box::new(Expression::new(
                    Expr::Boolean(Boolean { value: false }),
                    Type::int(),
                )),
                alternative: None,
            }),
            Type::int(),
        ),
    ];
    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_if_expr().unwrap())
    });
}

#[test]
fn parse_bool_test() {
    let inputs = ["true", "false"];
    let expects = [
        Expression::new(Expr::Boolean(Boolean { value: true }), Type::int()),
        Expression::new(Expr::Boolean(Boolean { value: false }), Type::int()),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_bool().unwrap())
    });
}

#[test]
fn parse_return_expr_test() {
    let inputs = ["return 10", "return true"];
    let expects = [
        Expression::new(
            Expr::ReturnExpr(ReturnExpr {
                value: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 10 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::ReturnExpr(ReturnExpr {
                value: Box::new(Expression::new(
                    Expr::Boolean(Boolean { value: true }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_return_expr().unwrap())
    });
}

#[test]
fn parse_unary_test() {
    let inputs = [
        "!true", "-10", "!!true", "-(-10)", "&x", "*p", "*&x", "******pp", "&&&&&&x",
    ];
    let expects = [
        Expression::new(
            Expr::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Not,
                expr: Box::new(Expression::new(
                    Expr::Boolean(Boolean { value: true }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Minus,
                expr: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 10 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Not,
                expr: Box::new(Expression::new(
                    Expr::UnaryExpr(UnaryExpr {
                        kind: UnaryExprKind::Not,
                        expr: Box::new(Expression::new(
                            Expr::Boolean(Boolean { value: true }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Minus,
                expr: Box::new(Expression::new(
                    Expr::UnaryExpr(UnaryExpr {
                        kind: UnaryExprKind::Minus,
                        expr: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 10 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Addr,
                expr: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "x".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Deref,
                expr: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "p".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Deref,
                expr: Box::new(Expression::new(
                    Expr::UnaryExpr(UnaryExpr {
                        kind: UnaryExprKind::Addr,
                        expr: Box::new(Expression::new(
                            Expr::Ident(Ident {
                                name: "x".to_string(),
                                offset: 8,
                                ty: Type::int(),
                            }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Deref,
                expr: Box::new(Expression::new(
                    Expr::UnaryExpr(UnaryExpr {
                        kind: UnaryExprKind::Deref,
                        expr: Box::new(Expression::new(
                            Expr::UnaryExpr(UnaryExpr {
                                kind: UnaryExprKind::Deref,
                                expr: Box::new(Expression::new(
                                    Expr::UnaryExpr(UnaryExpr {
                                        kind: UnaryExprKind::Deref,
                                        expr: Box::new(Expression::new(
                                            Expr::UnaryExpr(UnaryExpr {
                                                kind: UnaryExprKind::Deref,
                                                expr: Box::new(Expression::new(
                                                    Expr::Ident(Ident {
                                                        name: "pp".to_string(),
                                                        offset: 8,
                                                        ty: Type::int(),
                                                    }),
                                                    Type::int(),
                                                )),
                                            }),
                                            Type::int(),
                                        )),
                                    }),
                                    Type::int(),
                                )),
                            }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Addr,
                expr: Box::new(Expression::new(
                    Expr::UnaryExpr(UnaryExpr {
                        kind: UnaryExprKind::Addr,
                        expr: Box::new(Expression::new(
                            Expr::UnaryExpr(UnaryExpr {
                                kind: UnaryExprKind::Addr,
                                expr: Box::new(Expression::new(
                                    Expr::Ident(Ident {
                                        name: "x".to_string(),
                                        offset: 8,
                                        ty: Type::int(),
                                    }),
                                    Type::int(),
                                )),
                            }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![
            Ident {
                name: "x".to_string(),
                offset: 8,
                ty: Type::int(),
            },
            Ident {
                name: "p".to_string(),
                offset: 8,
                ty: Type::pointer_to(Box::new(Type::int())),
            },
            Ident {
                name: "pp".to_string(),
                offset: 8,
                ty: Type::pointer_to(Box::new(Type::pointer_to(Box::new(Type::pointer_to(
                    Box::new(Type::pointer_to(Box::new(Type::pointer_to(Box::new(
                        Type::pointer_to(Box::new(Type::int())),
                    ))))),
                ))))),
            },
        ];
        assert_eq!(expect, parser.parse_unary().unwrap())
    });
}

#[test]
fn parse_array_test() {
    let inputs = ["[1,2,3]", "[]"];
    let expects = [
        Expression::new(
            Expr::Array(Array {
                elements: vec![
                    Expression::new(Expr::Integer(Integer { value: 1 }), Type::int()),
                    Expression::new(Expr::Integer(Integer { value: 2 }), Type::int()),
                    Expression::new(Expr::Integer(Integer { value: 3 }), Type::int()),
                ],
            }),
            Type::pointer_to(Box::new(Type::int())),
        ),
        Expression::new(
            Expr::Array(Array { elements: vec![] }),
            Type::pointer_to(Box::new(Type::int())),
        ),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_array().unwrap())
    });
}

#[test]
fn parse_index_test() {
    let inputs = ["array[1]", "array[1+2]", "[1,2,3][0]", "[[1]][0][0]"];
    let expects = [
        Expression::new(
            Expr::IndexExpr(IndexExpr {
                array: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "array".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::pointer_to(Box::new(Type::int())),
                )),
                index: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 1 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::IndexExpr(IndexExpr {
                array: Box::new(Expression::new(
                    Expr::Ident(Ident {
                        name: "array".to_string(),
                        offset: 8,
                        ty: Type::int(),
                    }),
                    Type::pointer_to(Box::new(Type::int())),
                )),
                index: Box::new(Expression::new(
                    Expr::BinaryExpr(BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 1 }),
                            Type::int(),
                        )),
                        rhs: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 2 }),
                            Type::int(),
                        )),
                    }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::IndexExpr(IndexExpr {
                array: Box::new(Expression::new(
                    Expr::Array(Array {
                        elements: vec![
                            Expression::new(Expr::Integer(Integer { value: 1 }), Type::int()),
                            Expression::new(Expr::Integer(Integer { value: 2 }), Type::int()),
                            Expression::new(Expr::Integer(Integer { value: 3 }), Type::int()),
                        ],
                    }),
                    Type::pointer_to(Box::new(Type::int())),
                )),
                index: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 0 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
        Expression::new(
            Expr::IndexExpr(IndexExpr {
                array: Box::new(Expression::new(
                    Expr::IndexExpr(IndexExpr {
                        array: Box::new(Expression::new(
                            Expr::Array(Array {
                                elements: vec![Expression::new(
                                    Expr::Array(Array {
                                        elements: vec![Expression::new(
                                            Expr::Integer(Integer { value: 1 }),
                                            Type::int(),
                                        )],
                                    }),
                                    Type::pointer_to(Box::new(Type::int())),
                                )],
                            }),
                            Type::pointer_to(Box::new(Type::int())),
                        )),
                        index: Box::new(Expression::new(
                            Expr::Integer(Integer { value: 0 }),
                            Type::int(),
                        )),
                    }),
                    Type::pointer_to(Box::new(Type::int())),
                )),
                index: Box::new(Expression::new(
                    Expr::Integer(Integer { value: 0 }),
                    Type::int(),
                )),
            }),
            Type::int(),
        ),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        parser.local_vars = vec![Ident {
            name: "array".to_string(),
            offset: 8,
            ty: Type::pointer_to(Box::new(Type::int())),
        }];
        assert_eq!(expect, parser.parse_index().unwrap())
    });
}
