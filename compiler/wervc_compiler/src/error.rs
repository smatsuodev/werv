use wervc_type::{error::TypeCheckError, TypedExpression};

#[derive(Debug)]
pub enum CompileError {
    Unimplemented,
    ParserError(wervc_parser::parser::error::ParserError),
    InputIsNotProgram,
    NotLeftValue,
    ExpectedIdent { actual: TypedExpression },
    TypeCheckError(TypeCheckError),
}
