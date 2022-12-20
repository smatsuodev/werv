use crate::ast::Expression;

#[allow(deprecated)]
pub const NULL: Object = Object::_Null;

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    #[deprecated]
    /// Use NULL const instead of this
    _Null,
    Integer(isize),
    Boolean(bool),
    Function {
        /// Vector of ident
        params: Vec<Expression>,
        /// BlockExpr is only allowed to contain
        body: Expression,
    },
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                #[allow(deprecated)]
                Object::_Null => String::from("null"),
                Object::Integer(i) => i.to_string(),
                Object::Boolean(b) => b.to_string(),
                Object::Function { .. } => String::from("[Function]"),
            }
        )
    }
}
