#[derive(Debug, PartialEq)]
pub enum Node {
    Expr(Expr),
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

#[derive(Debug, PartialEq)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
}
