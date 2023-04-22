use wervc_ast::Expression;
use wervc_object::Object;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalError {
    UnexpectedObject(Object),
    UndefinedVariable(String),
    IdentRequired { actual: Expression },
    UnmatchedArgsLen { expected: usize, actual: usize },
    OutOfRange,
}
