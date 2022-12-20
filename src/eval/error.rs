#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvalError {
    EvalStatementError,
    EvalLetStatementError,
    EvalExpressionError,
    EvalUnaryExpressionError,
    EvalBinaryExpressionError,
    EvalIdentError,
}
