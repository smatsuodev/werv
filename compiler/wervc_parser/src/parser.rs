pub mod error;
#[cfg(test)]
mod test;

use self::error::ParserError;
use wervc_ast::{
    ty::{Type, TypeKind},
    Array, BinaryExpr, BinaryExprKind, BlockExpr, Boolean, CallExpr, Expr,
    Expression::{self},
    FunctionDefExpr, Ident, IfExpr, IndexExpr, Integer, LetExpr, Node, Program, ReturnExpr,
    Statement::{self},
    UnaryExpr, UnaryExprKind,
};
use wervc_lexer::{
    lexer::Lexer,
    token::{
        Token,
        TokenKind::{self, *},
    },
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    local_vars: Vec<Ident>,
    cur_offset: isize,
}

type PResult<T> = Result<T, ParserError>;

impl Parser {
    pub fn new(input: impl ToString) -> Parser {
        let lexer = Lexer::new(input);
        let mut parser = Parser {
            lexer,
            cur_token: Token::default(),
            local_vars: Vec::new(),
            cur_offset: 0,
        };

        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.lexer.next_token()
    }

    fn peek(&self, kind: TokenKind) -> bool {
        self.cur_token.kind == kind
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        if self.cur_token.kind == kind {
            self.next_token();
            return true;
        }

        false
    }

    fn expect(&mut self, kind: TokenKind) -> PResult<Token> {
        if self.cur_token.kind != kind {
            return Err(ParserError::UnexpectedToken {
                expected: kind,
                actual: self.cur_token.kind,
            });
        }

        let token = self.cur_token.clone();

        self.next_token();

        Ok(token)
    }

    fn create_ident(&mut self, ident: &Expression, ty: Type) -> PResult<Expression> {
        let Expr::Ident(ident) = &ident.expr else {
            return Err(ParserError::UnexpectedExpr(ident.clone()))
        };
        let name = ident.name.clone();

        let ident = Ident {
            name,
            offset: self.cur_offset + 8,
            ty,
        };

        self.cur_offset = ident.offset;
        self.local_vars.push(ident.clone());

        Ok(Expression::new(Expr::Ident(ident), ty))
    }

    fn find_ident(&self, ident: &Expression) -> PResult<Expression> {
        let original_ty = &ident.ty;
        let Expr::Ident(ident) = &ident.expr else {
            return Err(ParserError::UnexpectedExpr(ident.clone()))
        };
        let name = ident.name.clone();

        if let Some(ident) = self.local_vars.iter().rev().find_map(|ident| {
            if ident.name == name && &ident.ty == original_ty {
                Some(Expression::new(
                    Expr::Ident(ident.clone()),
                    ident.ty.clone(),
                ))
            } else {
                None
            }
        }) {
            Ok(ident)
        } else {
            Err(ParserError::UndefinedIdent(name))
        }
    }

    /// program = stmt*
    pub fn parse_program(&mut self) -> PResult<Node> {
        let mut statements = Vec::new();
        let mut is_returned = false;

        while !self.consume(EOF) {
            let stmt = self.parse_stmt()?;

            if is_returned {
                return Err(ParserError::RequiredSemiColon);
            }

            is_returned = matches!(stmt, Statement::ExprReturnStmt(_));
            statements.push(stmt);
        }

        Ok(Node::Program(Program {
            statements,
            total_offset: self.cur_offset,
        }))
    }

    /// stmt = expr ';'?
    fn parse_stmt(&mut self) -> PResult<Statement> {
        let expr = self.parse_expr()?;

        if self.consume(SemiColon) {
            return Ok(Statement::ExprStmt(expr));
        }

        Ok(Statement::ExprReturnStmt(expr))
    }

    /// expr = let_expr | if_expr | return_expr | assign
    fn parse_expr(&mut self) -> PResult<Expression> {
        if self.peek(Let) {
            return self.parse_let_expr();
        }

        if self.peek(If) {
            return self.parse_if_expr();
        }

        if self.peek(Return) {
            return self.parse_return_expr();
        }

        self.parse_assign()
    }

    /// let_expr = 'let' (ident ':' type | ident '(' (ident ':' type),* ')') '=' expr
    /// TODO: シャドーイングの実装
    fn parse_let_expr(&mut self) -> PResult<Expression> {
        self.expect(Let)?;

        let ident = self.parse_ident()?;

        if self.consume(LParen) {
            let mut params = Vec::new();

            if self.consume(RParen) {
                self.expect(Assign)?;

                let body = Box::new(self.parse_expr()?);
                let return_ty = Box::new(Type::calc_type(&body.expr));
                let name = Box::new(self.create_ident(&ident, Type::func(return_ty))?);
                let expr = Expr::FunctionDefExpr(FunctionDefExpr { name, params, body });

                return Ok(Expression::new(expr, Type::calc_type(&expr)));
            }

            let ident = self.parse_ident()?;

            self.expect(TokenKind::Colon)?;

            let ty = self.parse_type()?;
            let param = self.create_ident(&ident, ty)?;

            params.push(param);

            while self.consume(Comma) {
                let ident = self.parse_ident()?;

                self.expect(TokenKind::Colon)?;

                let ty = self.parse_type()?;
                let param = self.create_ident(&ident, ty)?;

                params.push(param);
            }

            self.expect(RParen)?;
            self.expect(Assign)?;

            let body = Box::new(self.parse_expr()?);
            let return_ty = Box::new(Type::calc_type(&body.expr));
            let name = Box::new(self.create_ident(&ident, Type::func(return_ty))?);
            let expr = Expr::FunctionDefExpr(FunctionDefExpr { name, params, body });

            return Ok(Expression::new(expr, Type::calc_type(&expr)));
        }

        self.expect(TokenKind::Colon)?;

        let ty = self.parse_type()?;
        let name = Box::new(self.create_ident(&ident, ty)?);

        self.expect(TokenKind::Assign)?;

        let value = Box::new(self.parse_expr()?);
        let expr = Expr::LetExpr(LetExpr { name, value });

        Ok(Expression::new(expr, Type::calc_type(&expr)))
    }

    /// if_expr = 'if' expr expr ('else' expr)?
    fn parse_if_expr(&mut self) -> PResult<Expression> {
        self.expect(If)?;

        let condition = Box::new(self.parse_expr()?);
        let consequence = Box::new(self.parse_expr()?);
        let alternative = if self.consume(Else) {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };
        let expr = Expr::IfExpr(IfExpr {
            condition,
            consequence,
            alternative,
        });

        Ok(Expression::new(expr, Type::calc_type(&expr)))
    }

    /// return_expr = 'return' expr
    fn parse_return_expr(&mut self) -> PResult<Expression> {
        self.expect(Return)?;

        let expr = Expr::ReturnExpr(ReturnExpr {
            value: Box::new(self.parse_expr()?),
        });

        Ok(Expression::new(expr, Type::calc_type(&expr)))
    }

    /// assign = relation ('=' relation)?
    fn parse_assign(&mut self) -> PResult<Expression> {
        let node = self.parse_relation()?;

        if self.consume(TokenKind::Assign) {
            let expr = Expr::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Assign,
                lhs: Box::new(node),
                rhs: Box::new(self.parse_relation()?),
            });

            return Ok(Expression::new(expr, Type::calc_type(&expr)));
        }

        Ok(node)
    }

    /// relation = add ('==' add | '!=' add | '<' add | '<=' add | '>' add | '>=' add)*
    fn parse_relation(&mut self) -> PResult<Expression> {
        let mut node = self.parse_add()?;

        loop {
            if self.consume(TokenKind::Eq) {
                let expr = Expr::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Eq,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });

                node = Expression::new(expr, Type::calc_type(&expr));
            } else if self.consume(TokenKind::Ne) {
                let expr = Expr::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Ne,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });

                node = Expression::new(expr, Type::calc_type(&expr));
            } else if self.consume(TokenKind::Lt) {
                let expr = Expr::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Lt,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });

                node = Expression::new(expr, Type::calc_type(&expr));
            } else if self.consume(TokenKind::Le) {
                let expr = Expr::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Le,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });

                node = Expression::new(expr, Type::calc_type(&expr));
            } else if self.consume(TokenKind::Gt) {
                let expr = Expr::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Gt,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });

                node = Expression::new(expr, Type::calc_type(&expr));
            } else if self.consume(TokenKind::Ge) {
                let expr = Expr::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Ge,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });

                node = Expression::new(expr, Type::calc_type(&expr));
            } else {
                return Ok(node);
            }
        }
    }

    /// add = mul ('+' mul | '-' mul)*
    fn parse_add(&mut self) -> PResult<Expression> {
        let mut node = self.parse_mul()?;

        loop {
            if self.consume(Plus) {
                let expr = Expr::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Add,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_mul()?),
                });

                node = Expression::new(expr, Type::calc_type(&expr));
            } else if self.consume(Minus) {
                let expr = Expr::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Sub,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_mul()?),
                });

                node = Expression::new(expr, Type::calc_type(&expr));
            } else {
                return Ok(node);
            }
        }
    }

    /// mul = unary ('*' unary | '/' unary )*
    fn parse_mul(&mut self) -> PResult<Expression> {
        let mut node = self.parse_unary()?;

        loop {
            if self.consume(Asterisk) {
                node = Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Mul,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_unary()?),
                });
            } else if self.consume(Slash) {
                node = Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Div,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_unary()?),
                });
            } else {
                return Ok(node);
            }
        }
    }

    /// unary = '!' unary | '*' unary | '&' unary | '-' index | index
    fn parse_unary(&mut self) -> PResult<Expression> {
        if self.consume(Bang) {
            return Ok(Expression::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Not,
                expr: Box::new(self.parse_unary()?),
            }));
        }

        if self.consume(Asterisk) {
            return Ok(Expression::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Deref,
                expr: Box::new(self.parse_unary()?),
            }));
        }

        if self.consume(Ampersand) {
            return Ok(Expression::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Addr,
                expr: Box::new(self.parse_unary()?),
            }));
        }

        if self.consume(Minus) {
            return Ok(Expression::UnaryExpr(UnaryExpr {
                kind: UnaryExprKind::Minus,
                expr: Box::new(self.parse_index()?),
            }));
        }

        self.parse_index()
    }

    /// index = call ('[' expr ']')*
    fn parse_index(&mut self) -> PResult<Expression> {
        let mut node = self.parse_call()?;

        while self.consume(LBracket) {
            let index = Box::new(self.parse_expr()?);

            node = Expression::IndexExpr(IndexExpr {
                array: Box::new(node),
                index,
            });
            self.expect(RBracket)?;
        }

        Ok(node)
    }

    /// call = primary ('(' expr,* ')')?
    fn parse_call(&mut self) -> PResult<Expression> {
        let node = self.parse_primary()?;

        if self.consume(LParen) {
            let mut args = Vec::new();

            if self.consume(RParen) {
                return Ok(Expression::CallExpr(CallExpr {
                    func: Box::new(node),
                    args,
                }));
            }

            args.push(self.parse_expr()?);

            while self.consume(Comma) {
                args.push(self.parse_expr()?);
            }

            self.expect(RParen)?;

            return Ok(Expression::CallExpr(CallExpr {
                func: Box::new(node),
                args,
            }));
        }

        Ok(node)
    }

    /// primary = '(' expr ')' | block_expr | array | integer | ident | bool
    fn parse_primary(&mut self) -> PResult<Expression> {
        if self.consume(LParen) {
            let expr = self.parse_expr()?;

            self.expect(RParen)?;

            return Ok(expr);
        }

        if self.peek(LBrace) {
            return self.parse_block_expr();
        }

        if self.peek(LBracket) {
            return self.parse_array();
        }

        if self.peek(Number) {
            return self.parse_integer();
        }

        if self.peek(TokenKind::Ident) {
            let ident = self.parse_ident()?;

            return self.find_ident(&ident);
        }

        self.parse_bool()
    }

    /// block_expr = '{' stmt* '}'
    fn parse_block_expr(&mut self) -> PResult<Expression> {
        self.expect(LBrace)?;

        let mut statements = Vec::new();

        while !self.consume(RBrace) {
            if self.consume(EOF) {
                return Err(ParserError::UnexpectedToken {
                    expected: RBrace,
                    actual: EOF,
                });
            }

            let stmt = self.parse_stmt()?;

            statements.push(stmt);
        }

        // Ok(BlockExpr(stmts))
        Ok(Expression::BlockExpr(BlockExpr { statements }))
    }

    /// array = '[' expr,* ']'
    fn parse_array(&mut self) -> PResult<Expression> {
        self.expect(LBracket)?;

        let mut elements = Vec::new();

        if self.consume(RBracket) {
            return Ok(Expression::Array(Array { elements }));
        }

        elements.push(self.parse_expr()?);

        while self.consume(Comma) {
            elements.push(self.parse_expr()?);
        }

        self.expect(RBracket)?;

        Ok(Expression::Array(Array { elements }))
    }

    /// integer = [0-9]*
    fn parse_integer(&mut self) -> PResult<Expression> {
        let token = self.expect(Number)?;
        let value = token
            .literal
            .parse::<isize>()
            .map_err(ParserError::ParseIntError)?;

        Ok(Expression::Integer(Integer { value }))
    }

    /// ident = ([a-zA-Z] | '_') ([a-zA-Z0-9] | '_')*
    fn parse_ident(&mut self) -> PResult<Expression> {
        let token = self.expect(TokenKind::Ident)?;

        Ok(Expression::Ident(Ident {
            name: token.literal,
            ty: Type {
                kind: TypeKind::Unknown,
                ptr_to: None,
            },
            offset: -1, // offsetは後から入れておくのでありえない値を入れとく
        }))
    }

    /// bool = 'true' | 'false'
    fn parse_bool(&mut self) -> PResult<Expression> {
        if self.consume(True) {
            return Ok(Expression::Boolean(Boolean { value: true }));
        }

        self.expect(False)?;

        Ok(Expression::Boolean(Boolean { value: false }))
    }

    fn parse_type(&mut self) -> PResult<Type> {
        let mut ptr_cnt = 0;

        while self.consume(TokenKind::Asterisk) {
            ptr_cnt += 1;
        }

        let type_name = self.expect(TokenKind::Ident)?.literal;
        let mut ty = Type {
            kind: TypeKind::from(type_name),
            ptr_to: None,
        };

        for _ in 0..ptr_cnt {
            ty = Type {
                kind: TypeKind::Ptr,
                ptr_to: Some(Box::new(ty)),
            }
        }

        Ok(ty)
    }
}
