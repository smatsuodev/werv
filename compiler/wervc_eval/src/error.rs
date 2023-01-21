use wervc_object::Object;

#[derive(Debug)]
pub enum EvalError {
    UnexpectedObject(Object),
}
