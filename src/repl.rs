use std::io::{stdin, stdout, Write};

use crate::{environment::Environment, eval::eval, lexer::Lexer, parser::Parser};

const PROMPT: &'static str = ">> ";

pub fn start() {
    let mut env = Environment::new();

    loop {
        print!("{PROMPT}");
        stdout().flush().expect("Failed to flush line");

        let mut line = String::new();

        stdin().read_line(&mut line).expect("Failed to read line");

        let lexer = Lexer::new(line);
        let mut parser = Parser::new(lexer);
        let program = match parser.parse() {
            Ok(p) => p,
            _ => {
                eprintln!("Parser error occurred");
                continue;
            }
        };
        let result = match eval(program, &mut env) {
            Ok(o) => o,
            _ => {
                eprintln!("Eval error occurred");
                continue;
            }
        };

        println!("{}", result);
    }
}
