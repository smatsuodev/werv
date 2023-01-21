use std::fmt::Display;
use Expr::*;
use Stmt::*;

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
impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExprStmt(e) => format!("{};", e),
                ExprReturnStmt(e) => e.to_string(),
            }
        )
    }
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
    CallExpr {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
}
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Integer(i) => i.to_string(),
                Ident(i) => i.clone(),
                BinaryExpr { kind, lhs, rhs } => format!("({} {} {})", lhs, kind, rhs),
                LetExpr { name, value } => format!("let {} = {}", name, value),
                BlockExpr(stmts) => format!(
                    "{{ {} }}",
                    stmts
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .join("; ")
                ),
                AssignExpr { name, value } => format!("{} = {}", name, value),
                CallExpr { func, args } => format!(
                    "{}({})",
                    func,
                    args.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
}
impl Display for BinaryExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Add => "+",
                Self::Sub => "-",
                Self::Mul => "*",
                Self::Div => "/",
            }
        )
    }
}
