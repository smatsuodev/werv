#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node {
    Program(Vec<Stmt>),
    Stmt(Stmt),
    Expr(Expr),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    ExprReturnStmt(Expr),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    Integer(isize),
    Ident(String),
    BinaryExpr {
        kind: BinaryExprKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    LetExpr {
        name: Box<Expr>,
        value: Box<Expr>,
    },
    BlockExpr(Vec<Stmt>),
    AssignExpr {
        name: Box<Expr>,
        value: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
}
