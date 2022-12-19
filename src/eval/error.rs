#[derive(Debug)]
pub enum EvalError {
    EvalStatementError,
    EvalLetStatementError,
    EvalExpressionError,
    EvalIfExpressionError,
    EvalUnaryExpressionError,
    EvalBinaryExpressionError,
}
