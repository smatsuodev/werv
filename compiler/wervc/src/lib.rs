use wervc_eval::{error::EvalError, Evaluator};
use wervc_object::Object;
use wervc_parser::parser::{error::ParserError, Parser};

#[derive(Debug)]
pub enum CompileError {
    ParserError(ParserError),
    EvalError(EvalError),
}

pub fn parse(input: &str) -> Result<Object, CompileError> {
    let mut parser = Parser::new(input);
    let program = parser.parse_program().map_err(CompileError::ParserError)?;
    let mut evaluator = Evaluator::new();

    evaluator.eval(program).map_err(CompileError::EvalError)
}
