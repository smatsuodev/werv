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
            TypeKind::Array {
                element_ty: element_type,
                size,
            } => element_type.calc_size() * size,
            TypeKind::Func { .. } => 8,
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
    pub fn func(params_ty: Vec<Type>, return_ty: Box<Type>) -> Type {
        Type {
            kind: TypeKind::Func {
                params_ty,
                return_ty,
            },
        }
    }
    pub fn array(element_type: Box<Type>, size: isize) -> Type {
        Type {
            kind: TypeKind::Array {
                element_ty: element_type,
                size,
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
        element_ty: Box<Type>,
        size: isize,
    },
}

impl<T: ToString> From<T> for Type {
    fn from(value: T) -> Self {
        match value.to_string().as_str() {
            "int" => Self::int(),
            "bool" => Self::bool(),
            _ => Self::unknown(),
        }
    }
}
