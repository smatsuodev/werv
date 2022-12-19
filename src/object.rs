#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    Ident(String),
}
