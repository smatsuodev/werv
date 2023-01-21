mod error;
#[cfg(test)]
mod test;

use error::EvalError;
use wervc_ast::{BinaryExprKind, Expr, Node};
use wervc_object::Object::{self, *};

type EResult = Result<Object, EvalError>;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {}
    }

    pub fn eval(&mut self, node: Node) -> EResult {
        match node {
            Node::Expr(e) => self.eval_expr(e),
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> EResult {
        match expr {
            Expr::BinaryExpr { kind, lhs, rhs } => self.eval_binary_expr(kind, *lhs, *rhs),
            Expr::Integer(i) => self.eval_integer(i),
        }
    }

    fn eval_binary_expr(&mut self, kind: BinaryExprKind, lhs: Expr, rhs: Expr) -> EResult {
        let lhs = self.eval_expr(lhs)?;
        let rhs = self.eval_expr(rhs)?;

        if let Integer(lhs) = lhs {
            if let Integer(rhs) = rhs {
                let value = match kind {
                    BinaryExprKind::Add => lhs + rhs,
                    BinaryExprKind::Sub => lhs - rhs,
                    BinaryExprKind::Mul => lhs * rhs,
                    BinaryExprKind::Div => lhs / rhs,
                };

                return Ok(Integer(value));
            }

            return Err(EvalError::UnexpectedObject(rhs));
        }

        Err(EvalError::UnexpectedObject(lhs))
    }

    fn eval_integer(&mut self, value: isize) -> EResult {
        Ok(Integer(value))
    }
}
