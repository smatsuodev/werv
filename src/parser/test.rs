use super::{
    ast::{ExprKind::*, Node::*},
    Parser,
};

#[test]
fn parse_expr() {
    let input = "(1 + (22 - 333) * 4444) / 55555";
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
                        rhs: Box::new(Integer(333))
                    }),
                    rhs: Box::new(Integer(4444))
                })
            }),
            rhs: Box::new(Integer(55555))
        })
    )
}
