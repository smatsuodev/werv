use wervc_ast::Expression;

#[derive(Debug)]
pub enum CompileError {
    Unimplemented,
    ParserError(wervc_parser::parser::error::ParserError),
    InputIsNotProgram,
    NotLeftValue,
    ExpectedIdent { actual: Expression },
}
