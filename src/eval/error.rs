#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvalError {
    EvalLetStatementError,
    EvalUnaryExpressionError,
    EvalBinaryExpressionError,
    EvalIdentError,
    EvalFunctionDefinitionStatementError,
    EvalCallExprError,
    EvalAssignExprError,
    EvalUpdateEnvError,
}

use EvalError::*;
impl ToString for EvalError {
    fn to_string(&self) -> String {
        let body = match self {
            EvalLetStatementError => "EvalLetStatementError",
            EvalUnaryExpressionError => "EvalUnaryExpressionError",
            EvalBinaryExpressionError => "EvalBinaryExpressionError",
            EvalIdentError => "EvalIdentError",
            EvalFunctionDefinitionStatementError => "EvalFunctionDefinitionStatementError",
            EvalCallExprError => "EvalCallExprError",
            EvalAssignExprError => "EvalAssignExprError",
            EvalUpdateEnvError => "EvalUpdateEnvError",
        };

        format!("Eval Error: {}", body)
    }
}
