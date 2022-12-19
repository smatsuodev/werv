#[derive(Debug)]
pub enum EvalError {
    EvalStatementError,
    EvalLetStatementError,
    EvalExpressionError,
    EvalUnaryExpressionError,
    EvalBinaryExpressionError,
    EvalIdentError,
}
