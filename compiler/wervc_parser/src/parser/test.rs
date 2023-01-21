use super::Parser;
use wervc_ast::{BinaryExprKind::*, Expr::*};

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
fn parse_integer_test() {
    let inputs = ["0", "42"];

    for input in inputs {
        let mut parser = Parser::new(input);

        assert_eq!(
            parser.parse_integer().unwrap(),
            Integer(input.parse::<isize>().unwrap())
        )
    }
}

#[test]
fn parse_binary_expr_test() {
    let inputs = ["1 + (2 - 3) * 4 / 5"];
    let expects = [BinaryExpr {
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

    loop_assert(inputs, expects, |parser, expect| {
        assert_eq!(expect, parser.parse_expr().unwrap())
    });
}
