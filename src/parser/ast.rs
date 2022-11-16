#[derive(Debug, PartialEq)]
pub enum Node {
    Expr {
        kind: ExprKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Integer(isize),
}

#[derive(Debug, PartialEq)]
pub enum ExprKind {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
