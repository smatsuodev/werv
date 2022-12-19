#[allow(deprecated)]
pub const NULL: Object = Object::_Null;

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    #[deprecated]
    _Null, // Use NULL const instead of this
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Integer(i) => i.to_string(),
                Object::Boolean(b) => b.to_string(),
                #[allow(deprecated)]
                Object::_Null => String::from("null"),
            }
        )
    }
}
