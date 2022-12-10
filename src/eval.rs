#[cfg(test)]
mod test;
use crate::{
    ast::{Expression, Node, Statement},
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
        Statement::ExprStatement(e) => eval(e),
        _ => Err(()),
    }
}

fn eval_expression(e: Expression) -> EResult {
    match e {
        Expression::Integer(i) => eval_integer(i),
        _ => Err(()),
    }
}

fn eval_integer(i: isize) -> EResult {
    Ok(Some(Integer(i)))
}
