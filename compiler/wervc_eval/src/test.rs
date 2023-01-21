use crate::{error::EvalError, EResult, Evaluator};
use wervc_ast::Expr;
use wervc_object::Object::{self, *};
use wervc_parser::parser::Parser;

fn loop_assert_unwrap<T, const N: usize>(inputs: [T; N], expects: [Object; N])
where
    T: ToString,
{
    for (input, expect) in inputs.into_iter().zip(expects) {
        let mut parser = Parser::new(input);
        let program = parser.parse_program().unwrap();
        let mut evaluator = Evaluator::new();

        assert_eq!(expect, evaluator.eval(program).unwrap());
    }
}

fn loop_assert<T, const N: usize>(inputs: [T; N], expects: [EResult; N])
where
    T: ToString,
{
    for (input, expect) in inputs.into_iter().zip(expects) {
        let mut parser = Parser::new(input);
        let program = parser.parse_program().unwrap();
        let mut evaluator = Evaluator::new();

        assert_eq!(expect, evaluator.eval(program));
    }
}

#[test]
fn eval_error_test() {
    let inputs = ["1+2 2+3;", "x;", "{ x }", "{ let x = 10; x }; x", "10 = 10"];
    let expects = [
        Err(EvalError::UnexpectedReturnedValue(Integer(3))),
        Err(EvalError::UndefinedVariable("x".to_string())),
        Err(EvalError::UndefinedVariable("x".to_string())),
        Err(EvalError::UndefinedVariable("x".to_string())),
        Err(EvalError::IdentRequired {
            got: Expr::Integer(10),
        }),
    ];

    loop_assert(inputs, expects);
}

#[test]
fn eval_integer_test() {
    let inputs = ["0", "42", "1234567890;"];
    let expects = [Integer(0), Integer(42), Unit];

    loop_assert_unwrap(inputs, expects);
}

#[test]
fn eval_arithmetic_test() {
    let inputs = [
        "1+2",
        "1-2",
        "2*3",
        "4/2",
        "1+2*3",
        "(1+2)*3",
        "1+2;",
        r"
        let x = 10;
        let y = 20;
        x + y
        ",
        r"
        { 10 } + { 20 }
        ",
    ];
    let expects = [
        Integer(3),
        Integer(-1),
        Integer(6),
        Integer(2),
        Integer(7),
        Integer(9),
        Unit,
        Integer(30),
        Integer(30),
    ];

    loop_assert_unwrap(inputs, expects);
}

#[test]
fn eval_let_expr_test() {
    let inputs = [
        "let x = 10;",
        "let x = 10",
        "let x = 10; x",
        "let x = 10; let x = 1; x",
        "let x = 10; let _123 = x; _123",
    ];
    let expects = [Unit, Integer(10), Integer(10), Integer(1), Integer(10)];

    loop_assert_unwrap(inputs, expects);
}

#[test]
fn eval_block_expr_test() {
    let inputs = [
        "{ 10 }",
        "{ 10; }",
        "{ let x = 10; x }",
        "let x = 10; { x }",
        "let x = 10; { let y = { x }; y }",
    ];
    let expects = [Integer(10), Unit, Integer(10), Integer(10), Integer(10)];

    loop_assert_unwrap(inputs, expects);
}

#[test]
fn eval_assign_expr_test() {
    let inputs = [
        "let x = 10; x = 20; x",
        "let x = 10; { x = 20; x }",
        "let x = 10; { x = 20; } x",
    ];
    let expects = [Integer(20), Integer(20), Integer(20)];

    loop_assert_unwrap(inputs, expects);
}
