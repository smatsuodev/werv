mod builtin;
mod environment;
pub mod error;
#[cfg(test)]
mod test;

use builtin::{call_builtin, is_builtin};
use environment::Environment;
use error::EvalError;
use wervc_ast::{BinaryExprKind, Expr, Node, Stmt, UnaryExprKind};
use wervc_object::Object::{self, *};

type EResult = Result<Object, EvalError>;

pub struct Evaluator {
    env: Environment,
}
impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            env: Environment::new(None),
        }
    }

    pub fn set_env(&mut self, env: Environment) {
        self.env = env;
    }

    pub fn set_outer(&mut self, outer: Environment) {
        self.env.set_outer(outer);
    }

    pub fn eval(&mut self, node: Node) -> EResult {
        match node {
            Node::Program(stmts) => {
                let value = self.eval_stmts(stmts)?;

                if let Return(value) = value {
                    return Ok(*value);
                }

                Ok(value)
            }
            Node::Stmt(stmt) => self.eval_stmt(stmt),
            Node::Expr(e) => self.eval_expr(e),
        }
    }

    fn eval_stmts(&mut self, stmts: Vec<Stmt>) -> EResult {
        let mut result = Unit;

        for stmt in stmts {
            let value = self.eval_stmt(stmt)?;

            if value.is_return() {
                return Ok(value);
            }

            result = value;
        }

        Ok(result)
    }

    fn eval_stmt(&mut self, stmt: Stmt) -> EResult {
        match stmt {
            Stmt::ExprStmt(e) => {
                let value = self.eval_expr(e)?;

                if value.is_return() {
                    return Ok(value);
                }

                Ok(Unit)
            }
            Stmt::ExprReturnStmt(e) => self.eval_expr(e),
        }
    }

    fn eval_expr(&mut self, expr: Expr) -> EResult {
        match expr {
            Expr::UnaryExpr { kind, expr } => self.eval_unary_expr(kind, *expr),
            Expr::ReturnExpr(e) => self.eval_return_expr(*e),
            Expr::Boolean(b) => self.eval_boolean(b),
            Expr::IfExpr {
                condition,
                consequence,
                alternative,
            } => self.eval_if_expr(*condition, *consequence, alternative),
            Expr::FunctionDefExpr { name, params, body } => {
                self.eval_function_def_expr(*name, params, *body)
            }
            Expr::CallExpr { func, args } => self.eval_call_expr(*func, args),
            Expr::AssignExpr { name, value } => self.eval_assign_expr(*name, *value),
            Expr::BlockExpr(stmts) => self.eval_block_expr(stmts),
            Expr::LetExpr { name, value } => self.eval_let_expr(*name, *value),
            Expr::Ident(i) => self.eval_ident(i),
            Expr::BinaryExpr { kind, lhs, rhs } => self.eval_binary_expr(kind, *lhs, *rhs),
            Expr::Integer(i) => self.eval_integer(i),
        }
    }

    fn eval_unary_expr(&mut self, kind: UnaryExprKind, expr: Expr) -> EResult {
        let value = self.eval_expr(expr)?;

        match kind {
            UnaryExprKind::Minus => {
                if let Integer(value) = value {
                    return Ok(Integer(-value));
                }
            }
            UnaryExprKind::Not => {
                if let Boolean(value) = value {
                    return Ok(Boolean(!value));
                }
            }
        }

        Err(EvalError::UnexpectedObject(value))
    }

    fn eval_return_expr(&mut self, e: Expr) -> EResult {
        Ok(Return(Box::new(self.eval_expr(e)?)))
    }

    fn eval_boolean(&mut self, value: bool) -> EResult {
        Ok(Boolean(value))
    }

    fn eval_if_expr(
        &mut self,
        condition: Expr,
        consequence: Expr,
        alternative: Option<Box<Expr>>,
    ) -> EResult {
        let condition = self.eval_expr(condition)?;

        if condition.is_return() {
            return Ok(condition);
        }

        if let Boolean(true) = condition {
            return self.eval_expr(consequence);
        } else if let Boolean(false) = condition {
            if let Some(alternative) = alternative {
                return self.eval_expr(*alternative);
            }

            return Ok(Unit);
        }

        Err(EvalError::UnexpectedObject(condition))
    }

    fn eval_function_def_expr(&mut self, name: Expr, params: Vec<Expr>, body: Expr) -> EResult {
        if let Expr::Ident(name) = &name {
            let params = params
                .iter()
                .map(|e| match e {
                    Expr::Ident(i) => i.clone(),
                    _ => panic!("Unexpected eval error: ident required but got {:?}", e),
                })
                .collect();
            let literal = Function { params, body };

            self.env.insert(name.clone(), literal.clone());

            return Ok(literal);
        }

        panic!("Unexpected eval error: ident required but got {:?}", name)
    }

    fn eval_call_expr(&mut self, func: Expr, args: Vec<Expr>) -> EResult {
        if is_builtin(&func) {
            let mut objects = Vec::new();

            for arg in args {
                let arg = self.eval_expr(arg)?;

                if arg.is_return() {
                    return Ok(arg);
                }

                objects.push(arg);
            }

            return Ok(call_builtin(&func, &objects).unwrap());
        }

        let func = self.eval_expr(func)?;

        if func.is_return() {
            return Ok(func);
        }

        if let Function { params, body } = &func {
            if args.len() != params.len() {
                return Err(EvalError::UnmatchedArgsLen {
                    expected: params.len(),
                    got: args.len(),
                });
            }

            let mut env = Environment::new(Some(Box::new(self.env.clone())));

            for (arg, param) in args.into_iter().zip(params) {
                let arg = self.eval_expr(arg)?;

                if arg.is_return() {
                    return Ok(arg);
                }

                env.insert(param.clone(), arg);
            }

            let mut inner = Evaluator::new();

            inner.set_env(env);

            let result = inner.eval_expr(body.clone())?;

            if let Return(result) = result {
                return Ok(*result);
            }

            return Ok(result);
        }

        Err(EvalError::UnexpectedObject(func))
    }

    fn eval_assign_expr(&mut self, name: Expr, value: Expr) -> EResult {
        if let Expr::Ident(name) = &name {
            let value = self.eval_expr(value)?;

            if value.is_return() {
                return Ok(value);
            }

            self.env
                .update(name.clone(), value.clone())
                .ok_or_else(|| EvalError::UndefinedVariable(name.clone()))?;

            return Ok(value);
        }

        if matches!(name, Expr::ReturnExpr(_)) {
            return self.eval_return_expr(name);
        }

        Err(EvalError::IdentRequired { got: name })
    }

    fn eval_block_expr(&mut self, stmts: Vec<Stmt>) -> EResult {
        // 内側のスコープ用に評価器を生成
        let mut inner = Evaluator::new();

        // 内側の環境のouterにブロックの外側のenvをクローン
        inner.set_outer(self.env.clone());

        let result = inner.eval_stmts(stmts)?;

        // 外側のenvに内側の環境のouterをムーブ
        self.set_env(inner.env.outer().unwrap());

        Ok(result)
    }

    fn eval_let_expr(&mut self, name: Expr, value: Expr) -> EResult {
        if let Expr::Ident(name) = name {
            let value = self.eval_expr(value)?;

            if value.is_return() {
                return Ok(value);
            }

            self.env.insert(name, value.clone());

            return Ok(value);
        }

        panic!("Unexpected eval error: ident required but got {:?}", name)
    }

    fn eval_ident(&mut self, name: String) -> EResult {
        if let Some(value) = self.env.get(&name) {
            return Ok(value.clone());
        }

        Err(EvalError::UndefinedVariable(name))
    }

    fn eval_binary_expr(&mut self, kind: BinaryExprKind, lhs: Expr, rhs: Expr) -> EResult {
        let lhs_obj = self.eval_expr(lhs)?;

        if lhs_obj.is_return() {
            return Ok(lhs_obj);
        }

        let rhs_obj = self.eval_expr(rhs)?;

        if rhs_obj.is_return() {
            return Ok(rhs_obj);
        }

        if kind == BinaryExprKind::Eq {
            return Ok(Boolean(lhs_obj == rhs_obj));
        }
        if kind == BinaryExprKind::Ne {
            return Ok(Boolean(lhs_obj != rhs_obj));
        }

        if let Integer(lhs) = lhs_obj {
            if let Integer(rhs) = rhs_obj {
                let value = match kind {
                    BinaryExprKind::Add => Integer(lhs + rhs),
                    BinaryExprKind::Sub => Integer(lhs - rhs),
                    BinaryExprKind::Mul => Integer(lhs * rhs),
                    BinaryExprKind::Div => Integer(lhs / rhs),
                    BinaryExprKind::Lt => Boolean(lhs < rhs),
                    BinaryExprKind::Le => Boolean(lhs <= rhs),
                    BinaryExprKind::Gt => Boolean(lhs > rhs),
                    BinaryExprKind::Ge => Boolean(lhs >= rhs),
                    _ => panic!(
                        "Unexpected eval error: {:?} operator has evaluated impossibly",
                        kind
                    ),
                };

                return Ok(value);
            }

            return Err(EvalError::UnexpectedObject(rhs_obj));
        }

        Err(EvalError::UnexpectedObject(lhs_obj))
    }

    fn eval_integer(&mut self, value: isize) -> EResult {
        Ok(Integer(value))
    }
}
