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
    Boolean(bool),
    Array(Vec<Expr>),
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
    CallExpr {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    FunctionDefExpr {
        name: Box<Expr>,
        params: Vec<Expr>,
        body: Box<Expr>,
    },
    IfExpr {
        condition: Box<Expr>,
        consequence: Box<Expr>,
        alternative: Option<Box<Expr>>,
    },
    ReturnExpr(Box<Expr>),
    UnaryExpr {
        kind: UnaryExprKind,
        expr: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryExprKind {
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
    Index,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryExprKind {
    Minus,
    Not,
}
