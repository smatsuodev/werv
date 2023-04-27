use wervc_eval::{error::EvalError, Evaluator};
use wervc_object::Object;
use wervc_parser::parser::{error::ParserError, Parser};

#[derive(Debug)]
pub enum InterpreterError {
    ParserError(ParserError),
    EvalError(EvalError),
}

pub struct Interpreter {
    evaluator: Evaluator,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            evaluator: Evaluator::new(),
        }
    }

    pub fn run(&mut self, input: &str) -> Result<Object, InterpreterError> {
        let mut parser = Parser::new(input);
        let program = parser
            .parse_program()
            .map_err(InterpreterError::ParserError)?;

        self.evaluator
            .eval(program)
            .map_err(InterpreterError::EvalError)
    }
}
