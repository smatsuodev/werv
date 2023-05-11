use crate::TypedExpression;
use wervc_ast::ty::Type;

#[derive(Debug)]
pub enum TypeCheckError {
    TypeError { expected: Type, actual: Type },
    AmbiguousTypeExprError(TypedExpression),
    NotCallableError(TypedExpression),
    NotIdentError(TypedExpression),
}
