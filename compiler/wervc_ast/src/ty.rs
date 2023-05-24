#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Type {
    pub kind: TypeKind,
}

impl Type {
    fn new(kind: TypeKind) -> Type {
        Type { kind }
    }
    pub fn calc_size(&self) -> isize {
        match &self.kind {
            TypeKind::Int => 8,
            TypeKind::Bool => 8,
            TypeKind::Ptr { .. } => 8,
            TypeKind::Func { .. } => 8,
            TypeKind::Array {
                element_type,
                length,
            } => element_type.calc_size() * length,
            TypeKind::Unknown => 0,
            TypeKind::Never => 0,
        }
    }
    pub fn is_assignable_to(&self, to: &Type) -> bool {
        if self == to {
            return true;
        }

        matches!(
            (&self.kind, &to.kind),
            (TypeKind::Int, TypeKind::Ptr { .. },)
        )
    }
    pub fn try_cast_to_ptr(&mut self) -> Option<&Type> {
        match &self.kind {
            TypeKind::Array { element_type, .. } => {
                *self = Type::pointer_to(element_type.clone());
                Some(self)
            }
            _ => None,
        }
    }

    pub fn int() -> Type {
        Type::new(TypeKind::Int)
    }
    pub fn bool() -> Type {
        Type::new(TypeKind::Bool)
    }
    pub fn unknown() -> Type {
        Type::new(TypeKind::Unknown)
    }
    pub fn never() -> Type {
        Type::new(TypeKind::Never)
    }
    pub fn pointer_to(ptr_to: Box<Type>) -> Type {
        Type {
            kind: TypeKind::Ptr { ptr_to },
        }
    }
    pub fn array(element_type: Box<Type>, length: isize) -> Type {
        Type {
            kind: TypeKind::Array {
                element_type,
                length,
            },
        }
    }
    pub fn func(params_ty: Vec<Type>, return_ty: Box<Type>) -> Type {
        Type {
            kind: TypeKind::Func {
                params_ty,
                return_ty,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeKind {
    Never,
    Unknown,
    Int,
    Bool,
    Func {
        params_ty: Vec<Type>,
        return_ty: Box<Type>,
    },
    Ptr {
        ptr_to: Box<Type>,
    },
    Array {
        element_type: Box<Type>,
        length: isize,
    },
}

impl<T: ToString> From<T> for TypeKind {
    fn from(value: T) -> Self {
        match value.to_string().as_str() {
            "int" => Self::Int,
            "bool" => Self::Bool,
            _ => Self::Unknown,
        }
    }
}
