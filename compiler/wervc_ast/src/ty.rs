use crate::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub ptr_to: Option<Box<Type>>,
}

impl Type {
    fn add_type(expr: &mut Expression) {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypeKind {
    Int,
    Ptr,
    Func,
    Unknown,
}

impl<T: ToString> From<T> for TypeKind {
    fn from(value: T) -> Self {
        match value.to_string().as_str() {
            "int" => Self::Int,
            _ => Self::Unknown,
        }
    }
}
