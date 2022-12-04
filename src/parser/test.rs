use crate::{
    ast::{Expression::*, Statement::*},
    lexer::Lexer,
};

use super::Parser;

#[test]
fn parse_let_stmt() {
    let inputs = ["let a = 10;"];
    let expects = [vec![LetStatement {
        name: Ident("a".to_string()),
        value: Integer(10),
    }]];

    for i in 0..inputs.len() {
        let l = Lexer::new(inputs[i].to_string());
        let mut p = Parser::new(l);

        assert_eq!(p.parse().unwrap(), expects[i]);
    }
}
