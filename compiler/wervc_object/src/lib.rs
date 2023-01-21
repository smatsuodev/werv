use wervc_ast::Expr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    Integer(isize),
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
                Self::FunctionLiteral { params, body } => {
                    format!("({}) => {body}", params.join(", "))
                }
                Self::Unit => "()".to_string(),
            }
        )
    }
}
