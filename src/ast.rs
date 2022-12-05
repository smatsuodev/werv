pub enum Node {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    LetStatement {
        name: Expression,
        value: Expression,
    },
    FunctionDefStatement {
        name: Expression,
        params: Vec<Expression>,
        body: Expression,
    },
    ExprStatement(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    CallExpr {
        name: Box<Expression>,
        args: Vec<Expression>,
    },
    BinaryExpr {
        kind: BinaryExprKind,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Ident(String),
    Integer(isize),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
}
