use wervc_eval::{error::EvalError, Evaluator};
use wervc_object::Object;
use wervc_parser::parser::{error::ParserError, Parser};

#[derive(Debug)]
pub enum CompileError {
    ParserError(ParserError),
    EvalError(EvalError),
}

pub struct Compiler {
    evaluator: Evaluator,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            evaluator: Evaluator::new(),
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<Object, CompileError> {
        let mut parser = Parser::new(input);
        let program = parser.parse_program().map_err(CompileError::ParserError)?;

        self.evaluator
            .eval(program)
            .map_err(CompileError::EvalError)
    }
}
