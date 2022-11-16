#[derive(Debug, PartialEq)]
pub enum Node {
    Assign {
        name: Box<Node>,
        expr: Box<Node>,
    },
    Expr {
        kind: ExprKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Ident(String),
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
