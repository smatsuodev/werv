use wervc_ast::Expr;
use wervc_object::Object;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalError {
    UnexpectedObject(Object),
    UnexpectedReturnedValue(Object),
    UndefinedVariable(String),
    IdentRequired { got: Expr },
}
