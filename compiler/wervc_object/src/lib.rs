use std::{borrow::BorrowMut, ops::DerefMut};

use wervc_ast::Expr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    Function { params: Vec<String>, body: Expr },
    Array(Vec<Object>),
    Return(Box<Object>),
    Pointer(Box<Object>),
    Unit,
}
impl Object {
    pub fn is_return(&self) -> bool {
        matches!(self, Self::Return(_))
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Integer(i) => i.to_string(),
                Self::Boolean(b) => b.to_string(),
                Self::Function { .. } => "[Function]".to_string(),
                Self::Array(values) => format!(
                    "[{}]",
                    values
                        .iter()
                        .map(|o| o.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
                Self::Return(o) => o.to_string(),
                Self::Pointer(p) => format!("{:p}", p),
                Self::Unit => "()".to_string(),
            }
        )
    }
}
