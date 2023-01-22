use super::{error::ParserError, Parser};
use wervc_ast::{BinaryExprKind::*, Expr::*, Stmt::*, UnaryExprKind};
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
    let inputs = ["{ 123", "10 10;"];
    let expects = [
        ParserError::UnexpectedToken {
            expected: TokenKind::RBrace,
            got: TokenKind::EOF,
        },
        ParserError::RequiredSemiColon,
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
        "let x = 1 + 2;",
        "let x = 1 + 2",
        "{ 10 };",
        "{ 10 }",
        "x = 1 + 2;",
        "x = 1 + 2",
    ];
    let expects = [
        ExprStmt(BinaryExpr {
            kind: Add,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(2)),
        }),
        ExprReturnStmt(BinaryExpr {
            kind: Add,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(2)),
        }),
        ExprStmt(LetExpr {
            name: Box::new(Ident("x".to_string())),
            value: Box::new(BinaryExpr {
                kind: Add,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(Integer(2)),
            }),
        }),
        ExprReturnStmt(LetExpr {
            name: Box::new(Ident("x".to_string())),
            value: Box::new(BinaryExpr {
                kind: Add,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(Integer(2)),
            }),
        }),
        ExprStmt(BlockExpr(vec![ExprReturnStmt(Integer(10))])),
        ExprReturnStmt(BlockExpr(vec![ExprReturnStmt(Integer(10))])),
        ExprStmt(AssignExpr {
            name: Box::new(Ident("x".to_string())),
            value: Box::new(BinaryExpr {
                kind: Add,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(Integer(2)),
            }),
        }),
        ExprReturnStmt(AssignExpr {
            name: Box::new(Ident("x".to_string())),
            value: Box::new(BinaryExpr {
                kind: Add,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(Integer(2)),
            }),
        }),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_stmt().unwrap())
    });
}

#[test]
fn parse_integer_test() {
    let inputs = ["0", "42"];
    let expects = [Integer(0), Integer(42)];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_integer().unwrap())
    });
}

#[test]
fn parse_binary_expr_test() {
    let inputs = ["1 + (2 - 3) * 4 / 5", "x + y"];
    let expects = [
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
        BinaryExpr {
            kind: Add,
            lhs: Box::new(Ident("x".to_string())),
            rhs: Box::new(Ident("y".to_string())),
        },
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_expr().unwrap())
    });
}

#[test]
fn parse_let_expr() {
    let inputs = [
        "let x = 1 + 2",
        "let foo_bar = x",
        "let _123 = _4567890",
        "let id(x) = x",
        "let add(x, y) = x + y",
        "let zero() = 0",
    ];
    let expects = [
        LetExpr {
            name: Box::new(Ident("x".to_string())),
            value: Box::new(BinaryExpr {
                kind: Add,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(Integer(2)),
            }),
        },
        LetExpr {
            name: Box::new(Ident("foo_bar".to_string())),
            value: Box::new(Ident("x".to_string())),
        },
        LetExpr {
            name: Box::new(Ident("_123".to_string())),
            value: Box::new(Ident("_4567890".to_string())),
        },
        FunctionDefExpr {
            name: Box::new(Ident("id".to_string())),
            params: vec![Ident("x".to_string())],
            body: Box::new(Ident("x".to_string())),
        },
        FunctionDefExpr {
            name: Box::new(Ident("add".to_string())),
            params: vec![Ident("x".to_string()), Ident("y".to_string())],
            body: Box::new(BinaryExpr {
                kind: Add,
                lhs: Box::new(Ident("x".to_string())),
                rhs: Box::new(Ident("y".to_string())),
            }),
        },
        FunctionDefExpr {
            name: Box::new(Ident("zero".to_string())),
            params: vec![],
            body: Box::new(Integer(0)),
        },
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_let_expr().unwrap())
    });
}

#[test]
fn parse_block_expr() {
    let inputs = [
        "{ 10 }",
        "{ let x = 10; x }",
        "{ let x = 10; }",
        "{ let x = { 10 } }",
        "{ return 10; }",
    ];
    let expects = [
        BlockExpr(vec![ExprReturnStmt(Integer(10))]),
        BlockExpr(vec![
            ExprStmt(LetExpr {
                name: Box::new(Ident("x".to_string())),
                value: Box::new(Integer(10)),
            }),
            ExprReturnStmt(Ident("x".to_string())),
        ]),
        BlockExpr(vec![ExprStmt(LetExpr {
            name: Box::new(Ident("x".to_string())),
            value: Box::new(Integer(10)),
        })]),
        BlockExpr(vec![ExprReturnStmt(LetExpr {
            name: Box::new(Ident("x".to_string())),
            value: Box::new(BlockExpr(vec![ExprReturnStmt(Integer(10))])),
        })]),
        BlockExpr(vec![ExprStmt(ReturnExpr(Box::new(Integer(10))))]),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_block_expr().unwrap())
    });
}

#[test]
fn parse_assign_test() {
    let inputs = ["x = 1 + 2", "x = y", "x = { 10 }"];
    let expects = [
        AssignExpr {
            name: Box::new(Ident("x".to_string())),
            value: Box::new(BinaryExpr {
                kind: Add,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(Integer(2)),
            }),
        },
        AssignExpr {
            name: Box::new(Ident("x".to_string())),
            value: Box::new(Ident("y".to_string())),
        },
        AssignExpr {
            name: Box::new(Ident("x".to_string())),
            value: Box::new(BlockExpr(vec![ExprReturnStmt(Integer(10))])),
        },
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_assign().unwrap())
    });
}

#[test]
fn parse_call_test() {
    let inputs = ["foo()", "foo(1,2,3)"];
    let expects = [
        CallExpr {
            func: Box::new(Ident("foo".to_string())),
            args: vec![],
        },
        CallExpr {
            func: Box::new(Ident("foo".to_string())),
            args: vec![Integer(1), Integer(2), Integer(3)],
        },
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_call().unwrap())
    });
}

#[test]
fn parse_relation_test() {
    let inputs = ["1 < 2", "1 <= 2", "1 > 2", "1 >= 2", "1 == 2", "1 != 2"];
    let expects = [
        BinaryExpr {
            kind: Lt,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(2)),
        },
        BinaryExpr {
            kind: Le,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(2)),
        },
        BinaryExpr {
            kind: Gt,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(2)),
        },
        BinaryExpr {
            kind: Ge,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(2)),
        },
        BinaryExpr {
            kind: Eq,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(2)),
        },
        BinaryExpr {
            kind: Ne,
            lhs: Box::new(Integer(1)),
            rhs: Box::new(Integer(2)),
        },
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
        IfExpr {
            condition: Box::new(BinaryExpr {
                kind: Lt,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(Integer(2)),
            }),
            consequence: Box::new(BlockExpr(vec![ExprReturnStmt(Integer(10))])),
            alternative: Some(Box::new(BlockExpr(vec![ExprReturnStmt(Integer(20))]))),
        },
        IfExpr {
            condition: Box::new(BinaryExpr {
                kind: Lt,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(Integer(2)),
            }),
            consequence: Box::new(BlockExpr(vec![ExprReturnStmt(Integer(10))])),
            alternative: None,
        },
        IfExpr {
            condition: Box::new(Boolean(true)),
            consequence: Box::new(Boolean(false)),
            alternative: None,
        },
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_if_expr().unwrap())
    });
}

#[test]
fn parse_bool_test() {
    let inputs = ["true", "false"];
    let expects = [Boolean(true), Boolean(false)];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_bool().unwrap())
    });
}

#[test]
fn parse_return_expr_test() {
    let inputs = ["return 10", "return true"];
    let expects = [
        ReturnExpr(Box::new(Integer(10))),
        ReturnExpr(Box::new(Boolean(true))),
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_return_expr().unwrap())
    });
}

#[test]
fn parse_unary_test() {
    let inputs = ["!true", "-10", "!!true", "-(-10)"];
    let expects = [
        UnaryExpr {
            kind: UnaryExprKind::Not,
            expr: Box::new(Boolean(true)),
        },
        UnaryExpr {
            kind: UnaryExprKind::Minus,
            expr: Box::new(Integer(10)),
        },
        UnaryExpr {
            kind: UnaryExprKind::Not,
            expr: Box::new(UnaryExpr {
                kind: UnaryExprKind::Not,
                expr: Box::new(Boolean(true)),
            }),
        },
        UnaryExpr {
            kind: UnaryExprKind::Minus,
            expr: Box::new(UnaryExpr {
                kind: UnaryExprKind::Minus,
                expr: Box::new(Integer(10)),
            }),
        },
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_unary().unwrap())
    });
}
