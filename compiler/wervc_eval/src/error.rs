use wervc_object::Object;

#[derive(Debug, PartialEq, Eq)]
pub enum EvalError {
    UnexpectedObject(Object),
    UnexpectedReturnedValue(Object),
}
