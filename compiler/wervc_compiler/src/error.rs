#[derive(Debug)]
pub enum CompileError {
    Unimplemented,
    ParserError(wervc_parser::parser::error::ParserError),
    InputIsNotProgram,
    NotLeftValue,
}
