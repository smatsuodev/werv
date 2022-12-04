pub enum Node {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    LetStatement { name: Expression, value: Expression },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryExpr {
        kind: BinaryExprKind,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Ident(String),
    Integer(isize),
}

#[derive(Debug, PartialEq)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
}
