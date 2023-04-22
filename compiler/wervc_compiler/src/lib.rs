pub mod error;

use std::fmt::Display;

use error::CompileError;
use wervc_ast::{BinaryExpr, BinaryExprKind, Expression, Integer, Node, Program, UnaryExpr};
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

    fn nullary(&mut self, operation: impl Display) {
        self.add_code(format!("  {}", operation));
    }

    fn unary_op(&mut self, operation: impl Display, operand: impl Display) {
        self.add_code(format!("  {} {}", operation, operand));
    }

    fn binary_op(&mut self, operation: impl Display, lhs: impl Display, rhs: impl Display) {
        self.add_code(format!("  {} {}, {}", operation, lhs, rhs));
    }

    fn push(&mut self, code: impl Display) {
        self.unary_op("push", code);
    }

    fn ret(&mut self) {
        self.nullary("ret");
    }

    fn pop(&mut self, code: impl Display) {
        self.unary_op("pop", code);
    }

    fn add(&mut self, lhs: impl Display, rhs: impl Display) {
        self.binary_op("add", lhs, rhs);
    }

    fn sub(&mut self, lhs: impl Display, rhs: impl Display) {
        self.binary_op("sub", lhs, rhs);
    }

    fn imul(&mut self, lhs: impl Display, rhs: impl Display) {
        self.binary_op("imul", lhs, rhs);
    }

    fn idiv(&mut self, value: impl Display) {
        self.nullary("cqo");
        self.unary_op("idiv", value);
    }

    fn neg(&mut self, value: impl Display) {
        self.unary_op("neg", value);
    }

    fn cmp(&mut self, lhs: impl Display, rhs: impl Display) {
        self.binary_op("cmp", lhs, rhs);
    }

    fn movzb(&mut self, lhs: impl Display, rhs: impl Display) {
        self.add_code(format!("  movzb {}, {}", lhs, rhs));
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
            Expression::BinaryExpr(e) => self.gen_binary_expr(e),
            Expression::UnaryExpr(e) => self.gen_unary_expr(e),
            _ => Err(CompileError::Unimplemented),
        }
    }

    fn gen_integer(&mut self, e: &Integer) -> CResult {
        self.push(e.value);
        Ok(())
    }

    fn gen_binary_expr(&mut self, e: &BinaryExpr) -> CResult {
        self.gen_expr(&e.lhs)?;
        self.gen_expr(&e.rhs)?;

        self.pop("rdi");
        self.pop("rax");

        match e.kind {
            BinaryExprKind::Add => {
                self.add("rax", "rdi");
            }
            BinaryExprKind::Sub => {
                self.sub("rax", "rdi");
            }
            BinaryExprKind::Mul => {
                self.imul("rax", "rdi");
            }
            BinaryExprKind::Div => {
                self.idiv("rdi");
            }
            BinaryExprKind::Eq => {
                self.cmp("rax", "rdi");
                self.unary_op("sete", "al");
                self.movzb("rax", "al");
            }
            BinaryExprKind::Ne => {
                self.cmp("rax", "rdi");
                self.unary_op("setne", "al");
                self.movzb("rax", "al");
            }
            BinaryExprKind::Ge => {
                self.cmp("rax", "rdi");
                self.unary_op("setge", "al");
                self.movzb("rax", "al");
            }
            BinaryExprKind::Gt => {
                self.cmp("rax", "rdi");
                self.unary_op("setg", "al");
                self.movzb("rax", "al");
            }
            BinaryExprKind::Le => {
                self.cmp("rax", "rdi");
                self.unary_op("setle", "al");
                self.movzb("rax", "al");
            }
            BinaryExprKind::Lt => {
                self.cmp("rax", "rdi");
                self.unary_op("setl", "al");
                self.movzb("rax", "al");
            }
            _ => {
                return Err(CompileError::Unimplemented);
            }
        }

        self.push("rax");

        Ok(())
    }

    fn gen_unary_expr(&mut self, e: &UnaryExpr) -> CResult {
        self.gen_expr(&e.expr)?;

        self.pop("rax");

        match e.kind {
            wervc_ast::UnaryExprKind::Minus => {
                self.neg("rax");
            }
            _ => {
                return Err(CompileError::Unimplemented);
            }
        }

        self.push("rax");

        Ok(())
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
