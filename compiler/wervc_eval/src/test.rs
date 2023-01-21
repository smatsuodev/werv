use crate::Evaluator;
use wervc_object::Object::*;
use wervc_parser::parser::Parser;

#[test]
fn eval_integer_test() {
    let inputs = ["0", "42"];

    let mut evaluator = Evaluator::new();

    for input in inputs {
        let mut parser = Parser::new(input);
        let program = parser.parse_program().unwrap();

        assert_eq!(
            evaluator.eval(program).unwrap(),
            Integer(input.parse::<isize>().unwrap())
        );
    }
}

#[test]
fn eval_arithmetic_test() {
    let inputs = ["1+2", "1-2", "2*3", "4/2"];
    let expects = [3, -1, 6, 2];

    let mut evaluator = Evaluator::new();

    for (input, expect) in inputs.into_iter().zip(expects) {
        let mut parser = Parser::new(input);
        let program = parser.parse_program().unwrap();

        assert_eq!(evaluator.eval(program).unwrap(), Integer(expect));
    }
}
