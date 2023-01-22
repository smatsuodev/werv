use wervc_ast::Expr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    Function { params: Vec<String>, body: Expr },
    Return(Box<Object>),
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
                Self::Return(o) => o.to_string(),
                Self::Unit => "()".to_string(),
            }
        )
    }
}
