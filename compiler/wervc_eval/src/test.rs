use crate::{error::EvalError, EResult, Evaluator};
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
    let inputs = ["1+2 2+3;"];
    let expects = [Err(EvalError::UnexpectedReturnedValue(Integer(3)))];

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
    let inputs = ["1+2", "1-2", "2*3", "4/2", "1+2*3", "(1+2)*3", "1+2;"];
    let expects = [
        Integer(3),
        Integer(-1),
        Integer(6),
        Integer(2),
        Integer(7),
        Integer(9),
        Unit,
    ];

    loop_assert_unwrap(inputs, expects);
}
