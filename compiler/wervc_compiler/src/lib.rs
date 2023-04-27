pub mod error;

use error::CompileError;
use std::fmt::Display;
use wervc_ast::{
    BinaryExpr, BinaryExprKind, BlockExpr, CallExpr, Expression, FunctionDefExpr, Integer, LetExpr,
    Node, Program, ReturnExpr, Statement, UnaryExpr, UnaryExprKind,
};
use wervc_parser::parser::Parser;

type CResult = Result<(), CompileError>;

const X86_64_ARG_REGISTERS: [&str; 6] = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];

/// アセンブリで数値に$をつけて表示するためのユーティリティ
trait IntoAssembly: ToString {
    fn to_asm(&self) -> String {
        self.to_string()
    }
}
impl IntoAssembly for isize {
    fn to_asm(&self) -> String {
        format!("${}", self)
    }
}
impl IntoAssembly for String {}
impl IntoAssembly for &String {}
impl IntoAssembly for &str {}

pub struct Compiler {
    pub outputs: Vec<String>,
    pub label_count: usize,
    // push/pop によって rsp が変化するので、その変化量を記録しておく
    // これは、関数の prologue/epilogue で rsp を調整するために必要
    pub depth: usize,
    pub cur_output_index: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            outputs: vec![String::new()],
            label_count: 0,
            depth: 0,
            cur_output_index: 0,
        }
    }

    pub fn output(&mut self) -> String {
        self.outputs.concat()
    }

    fn add_output(&mut self) {
        self.outputs.push(String::new());
    }

    fn change_output_to_end(&mut self) {
        self.cur_output_index = self.outputs.len() - 1;
    }

    fn change_output_to_head(&mut self) {
        self.cur_output_index = 0;
    }

    fn add_code(&mut self, code: impl ToString) {
        self.outputs[self.cur_output_index].push_str(code.to_string().as_str());
        self.outputs[self.cur_output_index].push('\n');
    }

    fn get_serial_label(&mut self, label: impl Display) -> String {
        let label = format!(".L{}{:>03}", label, self.label_count);

        self.label_count += 1;

        label
    }

    fn get_if_end_label(&mut self) -> String {
        self.get_serial_label("end")
    }

    fn get_if_else_label(&mut self) -> String {
        self.get_serial_label("else")
    }

    fn gen_label(&mut self, label: impl Display) {
        self.add_code(format!("{}:", label));
    }

    fn nullary(&mut self, operation: impl IntoAssembly) {
        self.add_code(format!("  {}", operation.to_asm()));
    }

    fn unary_op(&mut self, operation: impl IntoAssembly, operand: impl IntoAssembly) {
        self.add_code(format!("  {} {}", operation.to_asm(), operand.to_asm()));
    }

    fn binary_op(
        &mut self,
        operation: impl IntoAssembly,
        lhs: impl IntoAssembly,
        rhs: impl IntoAssembly,
    ) {
        self.add_code(format!(
            "  {} {}, {}",
            operation.to_asm(),
            lhs.to_asm(),
            rhs.to_asm()
        ));
    }

    fn mov(&mut self, lhs: impl IntoAssembly, rhs: impl IntoAssembly) {
        self.binary_op("mov", lhs, rhs);
    }

    fn call(&mut self, name: impl IntoAssembly) {
        self.unary_op("call", name);
    }

    fn ret(&mut self) {
        self.nullary("ret");
    }

    fn push(&mut self, from: impl IntoAssembly) {
        self.unary_op("push", from);
        self.depth += 1;
    }

    fn pop(&mut self, to: impl IntoAssembly) {
        self.unary_op("pop", to);
        self.depth -= 1;
    }

    fn add(&mut self, lhs: impl IntoAssembly, rhs: impl IntoAssembly) {
        self.binary_op("add", lhs, rhs);
    }

    fn sub(&mut self, lhs: impl IntoAssembly, rhs: impl IntoAssembly) {
        self.binary_op("sub", lhs, rhs);
    }

    fn imul(&mut self, lhs: impl IntoAssembly, rhs: impl IntoAssembly) {
        self.binary_op("imul", lhs, rhs);
    }

    fn idiv(&mut self, value: impl IntoAssembly) {
        self.nullary("cqo");
        self.unary_op("idiv", value);
    }

    fn neg(&mut self, value: impl IntoAssembly) {
        self.unary_op("neg", value);
    }

    fn cmp(&mut self, lhs: impl IntoAssembly, rhs: impl IntoAssembly) {
        self.binary_op("cmp", lhs, rhs);
    }

    fn movzb(&mut self, lhs: impl IntoAssembly, rhs: impl IntoAssembly) {
        self.binary_op("movzb", lhs, rhs);
    }

    fn je(&mut self, label: impl IntoAssembly) {
        self.unary_op("je", label);
    }

    fn jmp(&mut self, label: impl IntoAssembly) {
        self.unary_op("jmp", label);
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

        self.gen_program(&program)?;

        Ok(())
    }

    fn gen_prelude(&mut self) {
        self.add_code(".globl main");
        self.add_code("main:");
    }

    fn gen_program(&mut self, program: &Program) -> CResult {
        let Program {
            statements,
            total_offset,
        } = program;

        self.gen_prelude();
        self.gen_program_prologue(*total_offset);

        self.gen_statements(statements)?;

        self.push("%rax");
        self.gen_epilogue();

        Ok(())
    }

    fn gen_statements(&mut self, statements: &Vec<Statement>) -> CResult {
        for statement in statements {
            self.gen_statement(statement)?;
            self.pop("%rax");
        }

        Ok(())
    }

    fn gen_statement(&mut self, statement: &Statement) -> CResult {
        match statement {
            Statement::ExprStmt(e) => {
                self.gen_expr(e)?;
                self.pop("%rax");
                self.mov(0, "%rax");
            }
            Statement::ExprReturnStmt(e) => {
                self.gen_expr(e)?;
            }
        }

        Ok(())
    }

    fn gen_expr(&mut self, e: &Expression) -> CResult {
        match e {
            Expression::Integer(e) => self.gen_integer(e),
            Expression::BinaryExpr(e) => self.gen_binary_expr(e),
            Expression::UnaryExpr(e) => self.gen_unary_expr(e),
            Expression::Ident(_) => self.gen_ident(e),
            Expression::ReturnExpr(e) => self.gen_return_expr(e),
            Expression::IfExpr(e) => self.gen_if_expr(e),
            Expression::BlockExpr(e) => self.gen_block_expr(e),
            Expression::CallExpr(e) => self.gen_call_expr(e),
            Expression::FunctionDefExpr(e) => self.gen_function_def_expr(e),
            Expression::LetExpr(e) => self.gen_let_expr(e),
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

        self.pop("%rdi");
        self.pop("%rax");

        match e.kind {
            BinaryExprKind::Add => {
                self.add("%rdi", "%rax");
            }
            BinaryExprKind::Sub => {
                self.sub("%rdi", "%rax");
            }
            BinaryExprKind::Mul => {
                self.imul("%rdi", "%rax");
            }
            BinaryExprKind::Div => {
                self.idiv("%rdi");
            }
            BinaryExprKind::Eq => {
                self.cmp("%rdi", "%rax");
                self.unary_op("sete", "%al");
                self.movzb("%al", "%rax");
            }
            BinaryExprKind::Ne => {
                self.cmp("%rdi", "%rax");
                self.unary_op("setne", "%al");
                self.movzb("%al", "%rax");
            }
            BinaryExprKind::Ge => {
                self.cmp("%rdi", "%rax");
                self.unary_op("setge", "%al");
                self.movzb("%al", "%rax");
            }
            BinaryExprKind::Gt => {
                self.cmp("%rdi", "%rax");
                self.unary_op("setg", "%al");
                self.movzb("%al", "%rax");
            }
            BinaryExprKind::Le => {
                self.cmp("%rdi", "%rax");
                self.unary_op("setle", "%al");
                self.movzb("%al", "%rax");
            }
            BinaryExprKind::Lt => {
                self.cmp("%rdi", "%rax");
                self.unary_op("setl", "%al");
                self.movzb("%al", "%rax");
            }
            BinaryExprKind::Assign => {
                self.gen_left_val(&e.lhs)?;
                self.gen_expr(&e.rhs)?;

                self.pop("%rdi");
                self.pop("%rax");
                self.mov("%rdi", "(%rax)");
                self.push("%rdi");
            }
        }

        self.push("%rax");

        Ok(())
    }

    fn gen_unary_expr(&mut self, e: &UnaryExpr) -> CResult {
        self.gen_expr(&e.expr)?;

        self.pop("%rax");

        match e.kind {
            UnaryExprKind::Minus => {
                self.neg("%rax");
            }
            UnaryExprKind::Addr => {
                self.gen_left_val(&e.expr)?;
                return Ok(());
            }
            UnaryExprKind::Deref => {
                self.gen_expr(&e.expr)?;
                self.pop("%rax");
                self.mov("(%rax)", "%rax");
            }
            _ => {
                return Err(CompileError::Unimplemented);
            }
        }

        self.push("%rax");

        Ok(())
    }

    fn gen_left_val(&mut self, e: &Expression) -> CResult {
        match e {
            Expression::Ident(e) => {
                self.mov("%rbp", "%rax");
                self.sub(e.offset, "%rax");
                self.push("%rax");
            }
            _ => {
                return Err(CompileError::NotLeftValue);
            }
        }

        Ok(())
    }

    fn gen_ident(&mut self, e: &Expression) -> CResult {
        self.gen_left_val(e)?;
        self.pop("%rax");
        self.mov("(%rax)", "%rax");
        self.push("%rax");

        Ok(())
    }

    fn gen_program_prologue(&mut self, total_offset: isize) {
        self.push("%rbp");
        self.mov("%rsp", "%rbp");
        self.sub(total_offset, "%rsp");
    }

    fn gen_return_expr(&mut self, e: &ReturnExpr) -> CResult {
        self.gen_expr(&e.value)?;
        self.gen_epilogue();

        Ok(())
    }

    fn gen_if_expr(&mut self, e: &wervc_ast::IfExpr) -> CResult {
        self.gen_expr(&e.condition)?;
        self.pop("%rax");
        self.cmp(0, "%rax");

        if let Some(alternative) = &e.alternative {
            let else_label = self.get_if_else_label();
            let end_label = self.get_if_end_label();

            self.je(&else_label);
            self.gen_expr(&e.consequence)?;

            self.jmp(&end_label);
            self.gen_label(else_label);
            self.gen_expr(alternative)?;
            self.gen_label(end_label);
        } else {
            let end_label = self.get_if_end_label();

            self.je(&end_label);
            self.gen_expr(&e.consequence)?;
            self.gen_label(end_label);
        }

        Ok(())
    }

    fn gen_block_expr(&mut self, e: &BlockExpr) -> CResult {
        self.gen_statements(&e.statements)?;
        self.push("%rax");

        Ok(())
    }

    fn gen_call_expr(&mut self, e: &CallExpr) -> CResult {
        match &*e.func {
            Expression::Ident(func_name) => {
                let mut register_num = 0;

                for arg in &e.args {
                    self.gen_expr(arg)?;
                    register_num += 1;
                }

                for i in 0..register_num {
                    self.pop(X86_64_ARG_REGISTERS[register_num - i - 1]);
                }

                self.mov(0, "%rax");

                // rspを16バイト境界に揃える
                if self.depth % 2 == 0 {
                    self.call(&func_name.name);
                } else {
                    self.sub(8, "%rsp");
                    self.call(&func_name.name);
                    self.add(8, "%rsp");
                }

                self.push("%rax");
            }
            _ => {
                return Err(CompileError::Unimplemented);
            }
        }

        Ok(())
    }

    fn gen_function_def_expr(&mut self, e: &FunctionDefExpr) -> CResult {
        let func_name = match *e.name {
            Expression::Ident(ref i) => &i.name,
            _ => {
                return Err(CompileError::ExpectedIdent {
                    actual: *e.name.clone(),
                });
            }
        };

        self.add_output();
        self.change_output_to_end();
        self.add_code(format!(".globl {}", func_name));
        self.gen_label(func_name);
        self.push("%rbp");
        self.mov("%rsp", "%rbp");

        let mut max_offset = 0;

        for (i, param) in e.params.iter().enumerate() {
            if let Expression::Ident(param_ident) = param {
                max_offset = max_offset.max(param_ident.offset);

                // パラメータのオフセットを計算
                // 積むデータのサイズ分オフセットをずらす
                self.sub(param_ident.offset - 8, "%rsp");
                self.push(X86_64_ARG_REGISTERS[i]);
                self.mov("%rbp", "%rsp");
            } else {
                return Err(CompileError::ExpectedIdent {
                    actual: param.clone(),
                });
            }
        }

        self.sub(max_offset, "%rsp");
        self.gen_expr(&e.body)?;
        self.gen_epilogue();
        self.change_output_to_head();

        Ok(())
    }

    fn gen_epilogue(&mut self) {
        self.pop("%rax");
        self.mov("%rbp", "%rsp");
        self.pop("%rbp");
        self.ret();
    }

    fn gen_let_expr(&mut self, e: &LetExpr) -> CResult {
        self.gen_left_val(&e.name)?;
        self.gen_expr(&e.value)?;

        self.pop("%rdi");
        self.pop("%rax");
        self.mov("%rdi", "(%rax)");
        self.push("%rdi");
        self.push("%rax");

        Ok(())
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
