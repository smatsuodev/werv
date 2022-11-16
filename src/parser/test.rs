use std::borrow::Borrow;

use super::{
    ast::{ExprKind::*, Node::*},
    Parser,
};

#[test]
fn parse_expr() {
    let input = "(1 + (22 - (-333)) * -4444) / constant";
    let mut parser = Parser::new(input);

    assert_eq!(
        parser.parse().unwrap(),
        Box::new(Expr {
            kind: Div,
            lhs: Box::new(Expr {
                kind: Add,
                lhs: Box::new(Integer(1)),
                rhs: Box::new(Expr {
                    kind: Mul,
                    lhs: Box::new(Expr {
                        kind: Sub,
                        lhs: Box::new(Integer(22)),
                        rhs: Box::new(Expr {
                            kind: Sub,
                            lhs: Box::new(Integer(0)),
                            rhs: Box::new(Integer(333))
                        })
                    }),
                    rhs: Box::new(Expr {
                        kind: Sub,
                        lhs: Box::new(Integer(0)),
                        rhs: Box::new(Integer(4444))
                    })
                })
            }),
            rhs: Box::new(Ident("constant".to_string()))
        })
    )
}

#[test]
fn test_comparison() {
    let tests = [
        ("10 == 10", Eq),
        ("10 != 10", Ne),
        ("10 < 10", Lt),
        ("10 <= 10", Le),
        ("10 > 10", Gt),
        ("10 >= 10", Ge),
    ];

    for (input, kind) in tests {
        let mut parser = Parser::new(input);

        assert_eq!(
            parser.parse().unwrap(),
            Box::new(Expr {
                kind,
                lhs: Box::new(Integer(10)),
                rhs: Box::new(Integer(10))
            })
        )
    }
}

#[test]
fn test_ident() {
    let tests = ["abc", "ABC012", "_"];

    for input in tests {
        let mut parser = Parser::new(input);

        assert_eq!(parser.parse().unwrap(), Box::new(Ident(input.to_string())));
    }
}
