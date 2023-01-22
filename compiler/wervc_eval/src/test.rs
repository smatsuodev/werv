use crate::{error::EvalError, EResult, Evaluator};
use wervc_ast::{BinaryExprKind, Expr};
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
    let inputs = ["x;", "{ x }", "{ let x = 10; x }; x", "10 = 10", "if 1 1"];
    let expects = [
        Err(EvalError::UndefinedVariable("x".to_string())),
        Err(EvalError::UndefinedVariable("x".to_string())),
        Err(EvalError::UndefinedVariable("x".to_string())),
        Err(EvalError::IdentRequired {
            got: Expr::Integer(10),
        }),
        Err(EvalError::UnexpectedObject(Integer(1))),
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
        "let id(x) = x",
        "let id(x) = x; id(10)",
        "let add(x, y) = x + y",
        "let add(x, y) = x + y; add(10, 2)",
        "let one() = 1",
        "let one() = 1; one()",
        r"
        let fib(n) = {
            if n == 0 {
                return 0;
            };
            if n == 1 {
                return 1;
            };

            fib(n-1) + fib(n-2)
        };

        fib(10)
        ",
        r"
        let fact(n) = if n == 0 { 1 } else { n * fact(n-1) };
        
        fact(10)
        ",
    ];
    let expects = [
        Unit,
        Integer(10),
        Integer(10),
        Integer(1),
        Integer(10),
        Function {
            params: vec!["x".to_string()],
            body: Expr::Ident("x".to_string()),
        },
        Integer(10),
        Function {
            params: vec!["x".to_string(), "y".to_string()],
            body: Expr::BinaryExpr {
                kind: BinaryExprKind::Add,
                lhs: Box::new(Expr::Ident("x".to_string())),
                rhs: Box::new(Expr::Ident("y".to_string())),
            },
        },
        Integer(12),
        Function {
            params: vec![],
            body: Expr::Integer(1),
        },
        Integer(1),
        Integer(55),
        Integer(3628800),
    ];

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
        "let x = 10; { x = 20; }; x",
    ];
    let expects = [Integer(20), Integer(20), Integer(20)];

    loop_assert_unwrap(inputs, expects);
}

#[test]
fn eval_call_expr_test() {
    let inputs = ["print(10);", "println(20);", "let x = 10; print(x);"];
    let expects = [Unit, Unit, Unit];

    loop_assert_unwrap(inputs, expects);
}

#[test]
fn eval_if_expr_test() {
    let inputs = [
        "if true 10",
        "if false 10",
        "if true { 10 } else { 20 }",
        "if false { 10 } else { 20 }",
        "let x = 10; if x == 10 { 20 } else { 30 }",
        "let x = 10; if x == 20 { 20 } else { 30 }",
        "let x = 10; if x < 20 { 20 } else { 30 }",
        "let x = 10; if x > 20 { 20 } else { 30 }",
        "let x = 10; if x <= 20 { 20 } else { 30 }",
        "let x = 10; if x >= 20 { 20 } else { 30 }",
        "let x = 10; if x != 20 { 20 } else { 30 }",
        "let x = 10; if x != 10 { 20 } else { 30 }",
        "let x = 10; if x == 10 { 20 }",
        "let x = 10; if x == 20 { 20 }",
        "let x = 10; if x < 20 { 20 }",
        "let x = 10; if x > 20 { 20 }",
        "let x = 10; if x <= 20 { 20 }",
        "let x = 10; if x >= 20 { 20 }",
        "let x = 10; if x != 20 { 20 }",
        "let x = 10; if x != 10 { 20 }",
    ];
    let expects = [
        Integer(10),
        Unit,
        Integer(10),
        Integer(20),
        Integer(20),
        Integer(30),
        Integer(20),
        Integer(30),
        Integer(20),
        Integer(30),
        Integer(20),
        Integer(30),
        Integer(20),
        Unit,
        Integer(20),
        Unit,
        Integer(20),
        Unit,
        Integer(20),
        Unit,
    ];

    loop_assert_unwrap(inputs, expects);
}

#[test]
fn eval_return_expr_test() {
    let inputs = [
        "return 10",
        "let id(x) = { return x; }; id(10)",
        "(return 10) + 10",
        "if true { return 10 }; 20",
        "return 20; return 23;",
    ];
    let expects = [
        Integer(10),
        Integer(10),
        Integer(10),
        Integer(10),
        Integer(20),
    ];

    loop_assert_unwrap(inputs, expects);
}

#[test]
fn eval_unary_expr_test() {
    let inputs = ["-10", "-(-10)", "!true", "!false", "!!true", "!!false"];
    let expects = [
        Integer(-10),
        Integer(10),
        Boolean(false),
        Boolean(true),
        Boolean(true),
        Boolean(false),
    ];

    loop_assert_unwrap(inputs, expects);
}
