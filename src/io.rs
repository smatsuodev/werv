use std::fs;

use crate::{environment::Environment, eval::eval, lexer::Lexer, object::Object, parser::Parser};

pub fn execute_from_file(path: &str) -> Result<Object, String> {
    let code = fs::read_to_string(path).or(Err(String::from("Failed to read source file")))?;
    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let program = parser.parse().map_err(|e| e.to_string())?;
    let mut env = Environment::new();
    let result = eval(program, &mut env).map_err(|e| e.to_string())?;

    Ok(result)
}
