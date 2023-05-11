pub mod error;
#[cfg(test)]
mod test;

use error::TypeCheckError;
use wervc_ast::{
    ty::{Type, TypeKind},
    Array, BinaryExpr, BinaryExprKind, BlockExpr, Boolean, CallExpr, Expression, FunctionDefExpr,
    Ident, IfExpr, IndexExpr, Integer, LetExpr, Node, Program, ReturnExpr, Statement, UnaryExpr,
    UnaryExprKind,
};
use wervc_environment::Environment;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypedNode {
    Program(Program<TypedExpression>),
    Statement(Statement<TypedExpression>),
    Expression(TypedExpression),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TypedExpression {
    pub kind: TypedExpressionKind,
    pub ty: Type,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypedExpressionKind {
    Integer(Integer),
    Ident(Ident),
    Boolean(Boolean),
    Array(Array<TypedExpression>),
    BinaryExpr(BinaryExpr<TypedExpression>),
    LetExpr(LetExpr<TypedExpression>),
    BlockExpr(BlockExpr<TypedExpression>),
    CallExpr(CallExpr<TypedExpression>),
    FunctionDefExpr(FunctionDefExpr<TypedExpression>),
    IfExpr(IfExpr<TypedExpression>),
    ReturnExpr(ReturnExpr<TypedExpression>),
    UnaryExpr(UnaryExpr<TypedExpression>),
    IndexExpr(IndexExpr<TypedExpression>),
}

impl From<Expression> for TypedExpression {
    fn from(value: Expression) -> Self {
        let kind = match value {
            Expression::Integer(e) => TypedExpressionKind::Integer(e),
            Expression::Ident(e) => TypedExpressionKind::Ident(e),
            Expression::Boolean(e) => TypedExpressionKind::Boolean(e),
            Expression::Array(e) => TypedExpressionKind::Array(Array {
                elements: e.elements.into_iter().map(TypedExpression::from).collect(),
            }),
            Expression::BinaryExpr(e) => TypedExpressionKind::BinaryExpr(BinaryExpr {
                kind: e.kind,
                lhs: Box::new(TypedExpression::from(*e.lhs)),
                rhs: Box::new(TypedExpression::from(*e.rhs)),
            }),
            Expression::LetExpr(e) => TypedExpressionKind::LetExpr(LetExpr {
                name: Box::new(TypedExpression::from(*e.name)),
                value: Box::new(TypedExpression::from(*e.value)),
                ty: e.ty,
            }),
            Expression::BlockExpr(e) => TypedExpressionKind::BlockExpr(BlockExpr {
                statements: e
                    .statements
                    .into_iter()
                    .map(|s| match s {
                        Statement::ExprStmt(e) => Statement::ExprStmt(TypedExpression::from(e)),
                        Statement::ExprReturnStmt(e) => {
                            Statement::ExprReturnStmt(TypedExpression::from(e))
                        }
                    })
                    .collect(),
            }),
            Expression::CallExpr(e) => TypedExpressionKind::CallExpr(CallExpr {
                func: Box::new(TypedExpression::from(*e.func)),
                args: e.args.into_iter().map(TypedExpression::from).collect(),
            }),
            Expression::FunctionDefExpr(e) => {
                TypedExpressionKind::FunctionDefExpr(FunctionDefExpr {
                    name: Box::new(TypedExpression::from(*e.name)),
                    params: e
                        .params
                        .into_iter()
                        .map(|(e, ty)| (TypedExpression::from(e), ty))
                        .collect(),
                    return_ty: e.return_ty,
                    body: Box::new(TypedExpression::from(*e.body)),
                })
            }
            Expression::IfExpr(e) => TypedExpressionKind::IfExpr(IfExpr {
                condition: Box::new(TypedExpression::from(*e.condition)),
                consequence: Box::new(TypedExpression::from(*e.consequence)),
                alternative: e.alternative.map(|e| Box::new(TypedExpression::from(*e))),
            }),
            Expression::ReturnExpr(e) => TypedExpressionKind::ReturnExpr(ReturnExpr {
                value: Box::new(TypedExpression::from(*e.value)),
            }),
            Expression::UnaryExpr(e) => TypedExpressionKind::UnaryExpr(UnaryExpr {
                kind: e.kind,
                expr: Box::new(TypedExpression::from(*e.expr)),
            }),
            Expression::IndexExpr(e) => TypedExpressionKind::IndexExpr(IndexExpr {
                array: Box::new(TypedExpression::from(*e.array)),
                index: Box::new(TypedExpression::from(*e.index)),
            }),
        };

        TypedExpression {
            kind,
            ty: Type::unknown(),
        }
    }
}
impl From<TypedExpression> for Expression {
    fn from(val: TypedExpression) -> Self {
        match val.kind {
            TypedExpressionKind::Integer(e) => Expression::Integer(e),
            TypedExpressionKind::Ident(e) => Expression::Ident(e),
            TypedExpressionKind::Boolean(e) => Expression::Boolean(e),
            TypedExpressionKind::Array(e) => Expression::Array(Array {
                elements: e.elements.into_iter().map(TypedExpression::into).collect(),
            }),
            TypedExpressionKind::BinaryExpr(e) => Expression::BinaryExpr(BinaryExpr {
                kind: e.kind,
                lhs: Box::new(TypedExpression::into(*e.lhs)),
                rhs: Box::new(TypedExpression::into(*e.rhs)),
            }),
            TypedExpressionKind::LetExpr(e) => Expression::LetExpr(LetExpr {
                name: Box::new(TypedExpression::into(*e.name)),
                value: Box::new(TypedExpression::into(*e.value)),
                ty: e.ty,
            }),
            TypedExpressionKind::BlockExpr(e) => Expression::BlockExpr(BlockExpr {
                statements: e
                    .statements
                    .into_iter()
                    .map(|s| match s {
                        Statement::ExprStmt(e) => Statement::ExprStmt(TypedExpression::into(e)),
                        Statement::ExprReturnStmt(e) => {
                            Statement::ExprReturnStmt(TypedExpression::into(e))
                        }
                    })
                    .collect(),
            }),
            TypedExpressionKind::CallExpr(e) => Expression::CallExpr(CallExpr {
                func: Box::new(TypedExpression::into(*e.func)),
                args: e.args.into_iter().map(TypedExpression::into).collect(),
            }),
            TypedExpressionKind::FunctionDefExpr(e) => {
                Expression::FunctionDefExpr(FunctionDefExpr {
                    name: Box::new(TypedExpression::into(*e.name)),
                    params: e
                        .params
                        .into_iter()
                        .map(|(e, ty)| (TypedExpression::into(e), ty))
                        .collect(),
                    return_ty: e.return_ty,
                    body: Box::new(TypedExpression::into(*e.body)),
                })
            }
            TypedExpressionKind::IfExpr(e) => Expression::IfExpr(IfExpr {
                condition: Box::new(TypedExpression::into(*e.condition)),
                consequence: Box::new(TypedExpression::into(*e.consequence)),
                alternative: e.alternative.map(|e| Box::new(TypedExpression::into(*e))),
            }),
            TypedExpressionKind::ReturnExpr(e) => Expression::ReturnExpr(ReturnExpr {
                value: Box::new(TypedExpression::into(*e.value)),
            }),
            TypedExpressionKind::UnaryExpr(e) => Expression::UnaryExpr(UnaryExpr {
                kind: e.kind,
                expr: Box::new(TypedExpression::into(*e.expr)),
            }),
            TypedExpressionKind::IndexExpr(e) => Expression::IndexExpr(IndexExpr {
                array: Box::new(TypedExpression::into(*e.array)),
                index: Box::new(TypedExpression::into(*e.index)),
            }),
        }
    }
}

impl From<Node<Expression>> for TypedNode {
    fn from(value: Node<Expression>) -> Self {
        match value {
            Node::Program(p) => TypedNode::Program(Program {
                statements: p
                    .statements
                    .into_iter()
                    .map(|s| match s {
                        Statement::ExprStmt(e) => Statement::ExprStmt(TypedExpression::from(e)),
                        Statement::ExprReturnStmt(e) => {
                            Statement::ExprReturnStmt(TypedExpression::from(e))
                        }
                    })
                    .collect(),
            }),
            Node::Statement(s) => TypedNode::Statement(match s {
                Statement::ExprStmt(e) => Statement::ExprStmt(TypedExpression::from(e)),
                Statement::ExprReturnStmt(e) => Statement::ExprReturnStmt(TypedExpression::from(e)),
            }),
            Node::Expression(e) => TypedNode::Expression(TypedExpression::from(e)),
        }
    }
}

impl TypedNode {
    pub fn resolve_type(&mut self) -> Result<(Type, TypeResolver), TypeCheckError> {
        let mut resolver = TypeResolver::default();

        match self {
            TypedNode::Program(Program { statements }) => {
                let mut ty = Type::never();

                for stmt in statements {
                    ty = Self::resolve_type_stmt(stmt, &mut resolver)?;
                }

                Ok((ty, resolver))
            }
            TypedNode::Statement(stmt) => {
                Ok((Self::resolve_type_stmt(stmt, &mut resolver)?, resolver))
            }
            TypedNode::Expression(expr) => {
                Ok((Self::resolve_type_expr(expr, &mut resolver)?, resolver))
            }
        }
    }

    fn resolve_type_stmt(
        stmt: &mut Statement<TypedExpression>,
        resolver: &mut TypeResolver,
    ) -> Result<Type, TypeCheckError> {
        match stmt {
            Statement::ExprStmt(expr) => {
                Self::resolve_type_expr(expr, resolver)?;
            }
            Statement::ExprReturnStmt(expr) => {
                return Self::resolve_type_expr(expr, resolver);
            }
        }

        Ok(Type::never())
    }

    fn resolve_type_expr(
        expr: &mut TypedExpression,
        resolver: &mut TypeResolver,
    ) -> Result<Type, TypeCheckError> {
        resolver.resolve_type(expr)
    }
}

#[derive(Default)]
pub struct TypeResolver {
    pub local_vars: Environment<String, (Type, isize)>, // 型とオフセット
    pub cur_offset: isize,
}

impl TypeResolver {
    /// Return an type of an given ident. This may cause a panic if the ident not found because the ident is resolved in its name while parsing
    fn find_ident(&self, ident: &Ident) -> &(Type, isize) {
        self.local_vars.get_item(&ident.name).unwrap_or_else(|| {
            panic!(
                "ident not found: {:?}, local_vars: {:?}",
                ident, self.local_vars
            )
        })
    }

    fn create_ident(
        &mut self,
        ident: &mut TypedExpression,
        ty: Type,
    ) -> Result<Option<(Type, isize)>, TypeCheckError> {
        if let TypedExpressionKind::Ident(ident) = &mut ident.kind {
            self.cur_offset += ty.calc_size();
            ident.offset = self.cur_offset;

            Ok(self
                .local_vars
                .register_item(ident.name.clone(), (ty, ident.offset)))
        } else {
            Err(TypeCheckError::NotIdentError(ident.clone()))
        }
    }

    fn enter_scope(&mut self) {
        self.local_vars.create_deeper_scope();
    }

    fn leave_scope(&mut self) {
        self.local_vars.create_shallow_scope();
    }

    pub fn resolve_type(&mut self, expr: &mut TypedExpression) -> Result<Type, TypeCheckError> {
        match &mut expr.kind {
            TypedExpressionKind::Integer(_) => {
                expr.ty = Type::int();
            }
            TypedExpressionKind::Ident(ident) => {
                let (ident_ty, offset) = self.find_ident(ident);

                ident.offset = *offset;

                // identの型が不明な場合はエラー
                if *ident_ty == Type::unknown() {
                    return Err(TypeCheckError::AmbiguousTypeExprError(expr.clone()));
                }

                if expr.ty != Type::unknown() && expr.ty == *ident_ty {
                    return Err(TypeCheckError::TypeError {
                        expected: ident_ty.clone(),
                        actual: expr.ty.clone(),
                    });
                }

                expr.ty = ident_ty.clone();
            }
            TypedExpressionKind::Boolean(_) => {
                expr.ty = Type::bool();
            }
            TypedExpressionKind::BinaryExpr(BinaryExpr { kind, lhs, rhs }) => {
                self.resolve_type(lhs)?;
                self.resolve_type(rhs)?;

                match kind {
                    BinaryExprKind::Eq
                    | BinaryExprKind::Ne
                    | BinaryExprKind::Lt
                    | BinaryExprKind::Le
                    | BinaryExprKind::Gt
                    | BinaryExprKind::Ge => {
                        expr.ty = Type::bool();
                    }
                    BinaryExprKind::Add | BinaryExprKind::Sub => match (&lhs.ty.kind, &rhs.ty.kind)
                    {
                        (TypeKind::Int, TypeKind::Int) => {
                            expr.ty = lhs.ty.clone();
                        }
                        (TypeKind::Ptr { .. }, TypeKind::Int) => {
                            expr.ty = lhs.ty.clone();

                            // 左辺がポインタの場合は、右辺の整数×ポインタのサイズを加算する式に変換する
                            *rhs = Box::new(TypedExpression::from(Expression::BinaryExpr(
                                BinaryExpr {
                                    kind: BinaryExprKind::Mul,
                                    lhs: Box::new((*rhs.clone()).into()),
                                    rhs: Box::new(Expression::Integer(Integer {
                                        value: lhs.ty.calc_size(),
                                    })),
                                },
                            )));

                            self.resolve_type(&mut *rhs)?;
                        }
                        (TypeKind::Ptr { .. }, TypeKind::Ptr { .. }) => {
                            expr.ty = lhs.ty.clone();

                            // 両辺ともポインタの場合は、ポインタの間にいくつ要素があるか計算する式に変換する
                            // ポインタの差を取って、ポインタのサイズで割る
                            *rhs = Box::new(TypedExpression::from(Expression::BinaryExpr(
                                BinaryExpr {
                                    kind: BinaryExprKind::Div,
                                    lhs: Box::new(Expression::BinaryExpr(BinaryExpr {
                                        kind: BinaryExprKind::Sub,
                                        lhs: Box::new((*lhs.clone()).into()),
                                        rhs: Box::new((*rhs.clone()).into()),
                                    })),
                                    rhs: Box::new(Expression::Integer(Integer {
                                        value: lhs.ty.calc_size(),
                                    })),
                                },
                            )));

                            self.resolve_type(&mut *rhs)?;
                        }
                        _ => {
                            return Err(TypeCheckError::TypeError {
                                expected: Type::int(),
                                actual: lhs.ty.clone(),
                            });
                        }
                    },
                    BinaryExprKind::Assign => {
                        if !lhs.ty.is_assignable_to(&rhs.ty) {
                            return Err(TypeCheckError::TypeError {
                                expected: lhs.ty.clone(),
                                actual: rhs.ty.clone(),
                            });
                        }

                        expr.ty = lhs.ty.clone();
                    }
                    _ => {
                        expr.ty = lhs.ty.clone();
                    }
                };
            }
            TypedExpressionKind::LetExpr(LetExpr { name, value, ty }) => {
                self.resolve_type(value)?;

                // 左辺に代入できる型でなければエラー
                if !value.ty.is_assignable_to(ty) {
                    return Err(TypeCheckError::TypeError {
                        expected: ty.clone(),
                        actual: value.ty.clone(),
                    });
                }

                name.ty = ty.clone();
                expr.ty = ty.clone();

                self.create_ident(name, ty.clone())?;
            }
            TypedExpressionKind::BlockExpr(BlockExpr { statements }) => {
                let mut ty = Type::never();

                self.enter_scope();

                for stmt in statements {
                    ty = TypedNode::resolve_type_stmt(stmt, self)?;
                }

                expr.ty = ty;

                self.leave_scope();
            }
            TypedExpressionKind::CallExpr(CallExpr { func, args }) => {
                self.resolve_type(func)?;

                let TypeKind::Func { ref params_ty, ref return_ty } = func.ty.kind else {
                    return Err(TypeCheckError::NotCallableError(expr.clone()));
                };

                for (arg, param_ty) in args.iter_mut().zip(params_ty.iter()) {
                    self.resolve_type(arg)?;

                    if &arg.ty != param_ty {
                        return Err(TypeCheckError::TypeError {
                            expected: param_ty.clone(),
                            actual: arg.ty.clone(),
                        });
                    }
                }

                expr.ty = *return_ty.clone();
            }
            TypedExpressionKind::FunctionDefExpr(FunctionDefExpr {
                name,
                params,
                return_ty,
                body,
            }) => {
                let mut params_ty = Vec::new();

                for (param_ident, param_ty) in &mut *params {
                    param_ident.ty = param_ty.clone();
                    params_ty.push(param_ty.clone());
                }

                let func_ty = Type::func(params_ty, Box::new(return_ty.clone()));

                name.ty = func_ty.clone();
                expr.ty = func_ty.clone();

                self.create_ident(name, func_ty)?;

                self.enter_scope();

                for (param_ident, param_ty) in params {
                    self.create_ident(param_ident, param_ty.clone())?;
                }

                self.resolve_type(body)?;

                if body.ty != *return_ty {
                    return Err(TypeCheckError::TypeError {
                        expected: return_ty.clone(),
                        actual: body.ty.clone(),
                    });
                }

                self.leave_scope();
            }
            TypedExpressionKind::IfExpr(IfExpr {
                condition,
                consequence,
                alternative,
            }) => {
                self.resolve_type(condition)?;
                self.resolve_type(consequence)?;

                if condition.ty != Type::bool() && condition.ty != Type::int() {
                    return Err(TypeCheckError::TypeError {
                        expected: Type::bool(),
                        actual: condition.ty.clone(),
                    });
                }

                if let Some(alternative) = alternative.as_mut() {
                    self.resolve_type(alternative)?;

                    if consequence.ty != alternative.ty {
                        return Err(TypeCheckError::TypeError {
                            expected: consequence.ty.clone(),
                            actual: alternative.ty.clone(),
                        });
                    }
                }

                expr.ty = consequence.ty.clone();
            }
            TypedExpressionKind::ReturnExpr(ReturnExpr { value }) => {
                self.resolve_type(value)?;

                expr.ty = value.ty.clone();
            }
            TypedExpressionKind::UnaryExpr(UnaryExpr {
                kind,
                expr: unary_expr,
            }) => {
                self.resolve_type(unary_expr)?;

                match kind {
                    UnaryExprKind::Minus => {
                        if unary_expr.ty != Type::int() {
                            return Err(TypeCheckError::TypeError {
                                expected: Type::int(),
                                actual: unary_expr.ty.clone(),
                            });
                        }

                        unary_expr.ty = Type::int();
                    }
                    UnaryExprKind::Not => {
                        if unary_expr.ty != Type::bool() && unary_expr.ty != Type::int() {
                            return Err(TypeCheckError::TypeError {
                                expected: Type::bool(),
                                actual: unary_expr.ty.clone(),
                            });
                        }

                        unary_expr.ty = Type::bool();
                    }
                    UnaryExprKind::Deref => {
                        if let TypeKind::Ptr { ptr_to } = &unary_expr.ty.kind {
                            unary_expr.ty = *ptr_to.clone();
                        } else {
                            return Err(TypeCheckError::TypeError {
                                expected: Type::pointer_to(Box::new(Type::unknown())),
                                actual: unary_expr.ty.clone(),
                            });
                        }
                    }
                    UnaryExprKind::Addr => {
                        unary_expr.ty = Type::pointer_to(Box::new(unary_expr.ty.clone()));
                    }
                }

                expr.ty = unary_expr.ty.clone();
            }
            _ => panic!("unimplemented type of expression: {:?}", expr),
        }

        Ok(expr.ty.clone())
    }
}
