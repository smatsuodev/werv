pub mod error;

use std::fmt::{format, Display};

use error::CompileError;
use wervc_ast::{Expression, Integer, Node, Program};
use wervc_parser::parser::Parser;

type CResult = Result<(), CompileError>;

pub struct Compiler {
    pub output: String,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }

    fn add_code(&mut self, code: impl ToString) {
        self.output.push_str(code.to_string().as_str());
        self.output.push('\n');
    }

    fn push(&mut self, code: impl Display) {
        self.add_code(format!("  push {}", code));
    }

    fn ret(&mut self) {
        self.add_code("  ret");
    }

    fn pop(&mut self, code: impl Display) {
        self.add_code(format!("  pop {}", code));
    }

    pub fn compile(&mut self, program: impl ToString) -> CResult {
        let program = Parser::new(program)
            .parse_program()
            .map_err(CompileError::ParserError)?;
        let program = match program {
            Node::Program(p) => p,
            _ => {
                return Err(CompileError::InputIsNotProgram);
            }
        };

        self.gen_preload();
        self.gen_program(&program)?;

        self.pop("rax");
        self.ret();

        Ok(())
    }

    fn gen_preload(&mut self) {
        self.add_code(".intel_syntax noprefix");
        self.add_code(".globl main");
        self.add_code("main:");
    }

    fn gen_program(&mut self, program: &Program) -> CResult {
        let Program { statements } = program;

        for statement in statements {
            match statement {
                wervc_ast::Statement::ExprStmt(e) => {
                    return Err(CompileError::Unimplemented);
                }
                wervc_ast::Statement::ExprReturnStmt(e) => {
                    self.gen_expr(e)?;
                }
            }
        }

        Ok(())
    }

    fn gen_expr(&mut self, e: &Expression) -> CResult {
        match e {
            Expression::Integer(e) => self.gen_integer(e),
            _ => Err(CompileError::Unimplemented),
        }
    }

    fn gen_integer(&mut self, e: &Integer) -> CResult {
        self.push(e.value);
        Ok(())
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
