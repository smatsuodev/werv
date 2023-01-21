#[derive(Debug, PartialEq)]
pub enum Node {
    Program(Vec<Stmt>),
    Stmt(Stmt),
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ExprStmt(Expr),
    ExprReturnStmt(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Integer(isize),
    BinaryExpr {
        kind: BinaryExprKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
}
