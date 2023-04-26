mod builtin;
mod environment;
pub mod error;
#[cfg(test)]
mod test;

use builtin::{call_builtin, is_builtin};
use environment::Environment;
use error::EvalError;
use wervc_ast::{
    Array, BinaryExpr, BinaryExprKind, BlockExpr, Boolean, CallExpr, Expression, FunctionDefExpr,
    Ident, IfExpr, IndexExpr, Integer, LetExpr, Node, ReturnExpr, Statement, UnaryExpr,
    UnaryExprKind,
};
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
            Node::Program(program) => {
                let value = self.eval_stmts(program.statements)?;

                if let Return(value) = value {
                    return Ok(*value);
                }

                Ok(value)
            }
            Node::Statement(stmt) => self.eval_stmt(stmt),
            Node::Expression(e) => self.eval_expr(e),
        }
    }

    fn eval_stmts(&mut self, stmts: Vec<Statement>) -> EResult {
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

    fn eval_stmt(&mut self, stmt: Statement) -> EResult {
        match stmt {
            Statement::ExprStmt(e) => {
                let value = self.eval_expr(e)?;

                if value.is_return() {
                    return Ok(value);
                }

                Ok(Unit)
            }
            Statement::ExprReturnStmt(e) => self.eval_expr(e),
        }
    }

    fn eval_expr(&mut self, expr: Expression) -> EResult {
        match expr {
            Expression::Array(e) => self.eval_array(e),
            Expression::UnaryExpr(e) => self.eval_unary_expr(e),
            Expression::ReturnExpr(e) => self.eval_return_expr(e),
            Expression::Boolean(e) => self.eval_boolean(e),
            Expression::IfExpr(e) => self.eval_if_expr(e),
            Expression::Ident(e) => self.eval_ident(e),
            Expression::BinaryExpr(e) => self.eval_binary_expr(e),
            Expression::LetExpr(e) => self.eval_let_expr(e),
            Expression::BlockExpr(e) => self.eval_block_expr(e),
            Expression::CallExpr(e) => self.eval_call_expr(e),
            Expression::Integer(e) => self.eval_integer(e),
            Expression::FunctionDefExpr(e) => self.eval_function_def_expr(e),
            Expression::IndexExpr(e) => self.eval_index_expr(e),
        }
    }

    fn eval_array(&mut self, array: Array) -> EResult {
        let mut result = Vec::new();

        for element in array.elements {
            result.push(self.eval_expr(element)?)
        }

        Ok(Array(result))
    }

    fn eval_unary_expr(&mut self, unary: UnaryExpr) -> EResult {
        let value = self.eval_expr(*unary.expr)?;

        match unary.kind {
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
            UnaryExprKind::Deref => {
                if let Pointer(value) = value {
                    return Ok(*value);
                }
            }
            UnaryExprKind::Addr => {
                return Ok(Pointer(Box::new(value)));
            }
        }

        Err(EvalError::UnexpectedObject(value))
    }

    fn eval_return_expr(&mut self, return_expr: ReturnExpr) -> EResult {
        Ok(Return(Box::new(self.eval_expr(*return_expr.value)?)))
    }

    fn eval_boolean(&mut self, boolean: Boolean) -> EResult {
        Ok(Boolean(boolean.value))
    }

    fn eval_if_expr(&mut self, if_expr: IfExpr) -> EResult {
        let condition = self.eval_expr(*if_expr.condition)?;

        if condition.is_return() {
            return Ok(condition);
        }

        if let Boolean(true) = condition {
            return self.eval_expr(*if_expr.consequence);
        } else if let Boolean(false) = condition {
            if let Some(alternative) = if_expr.alternative {
                return self.eval_expr(*alternative);
            }

            return Ok(Unit);
        }

        Err(EvalError::UnexpectedObject(condition))
    }

    fn eval_function_def_expr(&mut self, func_def: FunctionDefExpr) -> EResult {
        if let Expression::Ident(Ident { name, .. }) = *func_def.name {
            let params = func_def
                .params
                .iter()
                .map(|e| match e {
                    Expression::Ident(Ident { name, .. }) => name.clone(),
                    _ => panic!("Unexpected eval error: ident required but got {:?}", e),
                })
                .collect();
            let literal = Function {
                params,
                body: *func_def.body,
            };

            self.env.insert(name, literal.clone());

            return Ok(literal);
        }

        panic!(
            "Unexpected eval error: ident required but got {:?}",
            func_def.name
        )
    }

    fn eval_call_expr(&mut self, CallExpr { func, args }: CallExpr) -> EResult {
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

        let func = self.eval_expr(*func)?;

        if func.is_return() {
            return Ok(func);
        }

        if let Function { params, body } = &func {
            if args.len() != params.len() {
                return Err(EvalError::UnmatchedArgsLen {
                    expected: params.len(),
                    actual: args.len(),
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

    fn eval_block_expr(&mut self, block_expr: BlockExpr) -> EResult {
        // 内側のスコープ用に評価器を生成
        let mut inner = Evaluator::new();

        // 内側の環境のouterにブロックの外側のenvをクローン
        inner.set_outer(self.env.clone());

        let result = inner.eval_stmts(block_expr.statements)?;

        // 外側のenvに内側の環境のouterをムーブ
        self.set_env(inner.env.outer().unwrap());

        Ok(result)
    }

    fn eval_let_expr(&mut self, let_expr: LetExpr) -> EResult {
        if let Expression::Ident(Ident { name, .. }) = *let_expr.name {
            let value = self.eval_expr(*let_expr.value)?;

            if value.is_return() {
                return Ok(value);
            }

            self.env.insert(name, value.clone());

            return Ok(value);
        }

        panic!(
            "Unexpected eval error: ident required but got {:?}",
            let_expr.name
        )
    }

    fn eval_ident(&mut self, ident: Ident) -> EResult {
        if let Some(value) = self.env.get(&ident.name) {
            return Ok(value.clone());
        }

        Err(EvalError::UndefinedVariable(ident.name))
    }

    fn eval_binary_expr(&mut self, binary_expr: BinaryExpr) -> EResult {
        let lhs = self.eval_expr(*binary_expr.lhs.clone())?;

        if lhs.is_return() {
            return Ok(lhs);
        }

        let rhs = self.eval_expr(*binary_expr.rhs.clone())?;

        if rhs.is_return() {
            return Ok(rhs);
        }

        let value = match binary_expr.kind {
            BinaryExprKind::Eq => Boolean(lhs == rhs),
            BinaryExprKind::Ne => Boolean(lhs != rhs),
            BinaryExprKind::Add => {
                if let Integer(lhs) = lhs {
                    if let Integer(rhs) = rhs {
                        return Ok(Integer(lhs + rhs));
                    }
                }

                return Err(EvalError::UnexpectedObject(rhs));
            }
            BinaryExprKind::Sub => {
                if let Integer(lhs) = lhs {
                    if let Integer(rhs) = rhs {
                        return Ok(Integer(lhs - rhs));
                    }
                }

                return Err(EvalError::UnexpectedObject(rhs));
            }
            BinaryExprKind::Mul => {
                if let Integer(lhs) = lhs {
                    if let Integer(rhs) = rhs {
                        return Ok(Integer(lhs * rhs));
                    }
                }

                return Err(EvalError::UnexpectedObject(rhs));
            }
            BinaryExprKind::Div => {
                if let Integer(lhs) = lhs {
                    if let Integer(rhs) = rhs {
                        return Ok(Integer(lhs / rhs));
                    }
                }

                return Err(EvalError::UnexpectedObject(rhs));
            }
            BinaryExprKind::Lt => {
                if let Integer(lhs) = lhs {
                    if let Integer(rhs) = rhs {
                        return Ok(Boolean(lhs < rhs));
                    }
                }

                return Err(EvalError::UnexpectedObject(rhs));
            }
            BinaryExprKind::Le => {
                if let Integer(lhs) = lhs {
                    if let Integer(rhs) = rhs {
                        return Ok(Boolean(lhs <= rhs));
                    }
                }

                return Err(EvalError::UnexpectedObject(rhs));
            }
            BinaryExprKind::Gt => {
                if let Integer(lhs) = lhs {
                    if let Integer(rhs) = rhs {
                        return Ok(Boolean(lhs > rhs));
                    }
                }

                return Err(EvalError::UnexpectedObject(rhs));
            }
            BinaryExprKind::Ge => {
                if let Integer(lhs) = lhs {
                    if let Integer(rhs) = rhs {
                        return Ok(Boolean(lhs >= rhs));
                    }
                }

                return Err(EvalError::UnexpectedObject(rhs));
            }
            BinaryExprKind::Assign => {
                if let Expression::Ident(Ident { name, .. }) = *binary_expr.lhs {
                    self.env
                        .update(name.clone(), rhs.clone())
                        .ok_or_else(|| EvalError::UndefinedVariable(name.clone()))?;

                    return Ok(rhs);
                }

                return Err(EvalError::IdentRequired {
                    actual: *binary_expr.lhs,
                });
            }
        };

        Ok(value)
    }

    fn eval_integer(&mut self, integer: Integer) -> EResult {
        Ok(Integer(integer.value))
    }

    fn eval_index_expr(&mut self, index_expr: IndexExpr) -> Result<Object, EvalError> {
        let array = self.eval_expr(*index_expr.array)?;
        let index = self.eval_expr(*index_expr.index)?;

        if let Array(elements) = array {
            if let Integer(index) = index {
                if let Ok(index) = index.try_into() {
                    return elements.into_iter().nth(index).ok_or(EvalError::OutOfRange);
                }

                let index: usize = index.abs().try_into().unwrap();

                if index > elements.len() {
                    return Err(EvalError::OutOfRange);
                }

                let index: usize = elements.len() - index;

                return elements.into_iter().nth(index).ok_or(EvalError::OutOfRange);
            }

            return Err(EvalError::UnexpectedObject(index));
        }

        Err(EvalError::UnexpectedObject(array))
    }
}
