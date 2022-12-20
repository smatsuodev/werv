#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvalError {
    EvalLetStatementError,
    EvalUnaryExpressionError,
    EvalBinaryExpressionError,
    EvalIdentError,
    EvalFunctionDefinitionStatementError,
    EvalCallExprError,
}
