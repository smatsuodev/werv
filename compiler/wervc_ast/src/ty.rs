use crate::{Expr, Expression, Statement};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub ptr_to: Option<Box<Type>>,
    pub return_ty: Option<Box<Type>>, // use if kind is Func
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeKind {
    Nil, // 何も返さない。Statementは常にNilを返す
    Unknown,

    Int,
    Ptr,
    Func,
}

impl Type {
    fn new(kind: TypeKind) -> Type {
        Type {
            kind,
            ptr_to: None,
            return_ty: None,
        }
    }

    pub fn nil() -> Type {
        Type::new(TypeKind::Nil)
    }

    pub fn int() -> Type {
        Type::new(TypeKind::Int)
    }

    pub fn pointer_to(ty: Box<Type>) -> Type {
        Type {
            kind: TypeKind::Int,
            ptr_to: Some(ty),
            return_ty: None,
        }
    }

    pub fn func(return_ty: Box<Type>) -> Type {
        Type {
            kind: TypeKind::Func,
            ptr_to: None,
            return_ty: Some(return_ty),
        }
    }

    /// 式として評価したときに返す型を計算する
    pub fn calc_type(expr: &Expr) -> Type {
        match &expr {
            Expr::Integer(_) => Type::int(),
            Expr::Ident(e) => e.ty.clone(),
            Expr::Boolean(_) => Type::int(),
            Expr::BinaryExpr(e) => Self::calc_type(&e.lhs.expr),
            Expr::LetExpr(e) => Self::calc_type(&e.name.expr),
            Expr::BlockExpr(e) => {
                if e.statements.is_empty() {
                    Type::nil()
                } else if let Statement::ExprReturnStmt(e) = &e.statements.last().unwrap() {
                    Self::calc_type(&e.expr)
                } else {
                    Type::nil()
                }
            }
            Expr::CallExpr(e) => *Self::calc_type(&e.func.expr).return_ty.unwrap(),
            Expr::FunctionDefExpr(e) => Type::func(Box::new(Self::calc_type(&e.body.expr))),
            Expr::IfExpr(e) => Self::calc_type(&e.consequence.expr),
            Expr::ReturnExpr(e) => Self::calc_type(&e.value.expr),
            Expr::UnaryExpr(e) => Self::calc_type(&e.expr.expr),
            _ => panic!("type of {:?} is not implemented", expr),
        }
    }
}

impl<T: ToString> From<T> for TypeKind {
    fn from(value: T) -> Self {
        match value.to_string().as_str() {
            "int" => Self::Int,
            _ => Self::Unknown,
        }
    }
}
