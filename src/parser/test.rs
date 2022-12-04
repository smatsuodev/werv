use crate::{
    ast::{BinaryExprKind::*, Expression::*, Statement::*},
    lexer::Lexer,
};

use super::Parser;

#[test]
fn parse_let_stmt() {
    let inputs = ["let a = 10;", "let b = 1 + (2 - 3) * 4 / 5;"];
    let expects = [
        vec![LetStatement {
            name: Ident("a".to_string()),
            value: Integer(10),
        }],
        vec![LetStatement {
            name: Ident("b".to_string()),
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
        }],
    ];

    for i in 0..inputs.len() {
        let l = Lexer::new(inputs[i].to_string());
        let mut p = Parser::new(l);

        assert_eq!(p.parse().unwrap(), expects[i]);
    }
}
