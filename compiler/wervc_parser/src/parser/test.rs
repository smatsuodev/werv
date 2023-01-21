use super::Parser;
use wervc_ast::{BinaryExprKind::*, Expr::*, Stmt::*};

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
fn parse_stmt_test() {
    let inputs = ["1 + 2;", "1 + 2", "let x = 1 + 2;", "let x = 1 + 2"];
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
    let inputs = ["let x = 1 + 2", "let foo_bar = x", "let _123 = _4567890"];
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
    ];

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_let_expr().unwrap())
    });
}
