#[cfg(test)]
mod test;
use crate::{
    ast::{BinaryExprKind, Expression, Node, Statement, UnaryExprKind},
    object::{Object, Object::*},
};

type EResult = Result<Option<Object>, ()>;

pub fn eval(node: impl Into<Node>) -> EResult {
    let node: Node = node.into();

    match node {
        Node::Program(stmts) => eval_statements(stmts),
        Node::Statement(s) => eval_statement(s),
        Node::Expression(e) => eval_expression(e),
    }
}

fn eval_statements(stmts: Vec<Statement>) -> EResult {
    let mut result = None;

    for s in stmts {
        result = eval(s)?;
    }

    Ok(result)
}

fn eval_statement(s: Statement) -> EResult {
    match s {
        // TODO: 今はデバッグのためにExprStatementが値を返すようになっているが、本来は返さない
        Statement::ExprStatement(e) => eval(e),
        _ => Err(()),
    }
}

fn eval_expression(e: Expression) -> EResult {
    match e {
        Expression::UnaryExpr { kind, expr } => eval_unary_expr(kind, expr),
        Expression::BinaryExpr { kind, lhs, rhs } => eval_binary_expr(kind, lhs, rhs),
        Expression::Integer(i) => eval_integer(i),
        Expression::Boolean(b) => eval_boolean(b),
        _ => Err(()),
    }
}

fn eval_unary_expr(kind: UnaryExprKind, expr: Box<Expression>) -> EResult {
    let expr = eval(*expr)?.ok_or(())?;

    if let (UnaryExprKind::Not, Object::Boolean(b)) = (kind, &expr) {
        return eval_boolean(!b);
    }
    if let (UnaryExprKind::Minus, Object::Integer(i)) = (kind, &expr) {
        return eval_integer(-i);
    }

    Err(())
}

fn eval_binary_expr(kind: BinaryExprKind, lhs: Box<Expression>, rhs: Box<Expression>) -> EResult {
    let lhs = eval(*lhs)?.ok_or(())?;
    let rhs = eval(*rhs)?.ok_or(())?;

    if let (Integer(lhs), Integer(rhs)) = (lhs, rhs) {
        let result = match kind {
            BinaryExprKind::Add => Integer(lhs + rhs),
            BinaryExprKind::Sub => Integer(lhs - rhs),
            BinaryExprKind::Mul => Integer(lhs * rhs),
            BinaryExprKind::Div => Integer(lhs / rhs),
            BinaryExprKind::Mod => Integer(lhs % rhs),
        };

        return Ok(Some(result));
    }

    Err(())
}

fn eval_integer(i: isize) -> EResult {
    Ok(Some(Integer(i)))
}

fn eval_boolean(b: bool) -> EResult {
    Ok(Some(Boolean(b)))
}
