use wervc_ast::Expr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    FunctionLiteral { params: Vec<String>, body: Expr },
    Unit,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Integer(i) => i.to_string(),
                Self::Boolean(b) => b.to_string(),
                Self::FunctionLiteral { .. } => "[Function]".to_string(),
                Self::Unit => "()".to_string(),
            }
        )
    }
}
