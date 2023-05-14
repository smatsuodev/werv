pub mod error;
#[cfg(test)]
mod test;

use self::error::ParserError;
use wervc_ast::{
    ty::Type,
    Array, BinaryExpr, BinaryExprKind, BlockExpr, Boolean, CallExpr,
    Expression::{self},
    FunctionDefExpr, Ident, IfExpr, IndexExpr, Integer, LetExpr, Node, Program, ReturnExpr,
    Statement::{self},
    UnaryExpr, UnaryExprKind,
};
use wervc_environment::Environment;
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
    local_vars: Environment<String, Ident>,
}

type PResult<T> = Result<T, ParserError>;

impl Parser {
    pub fn new(input: impl ToString) -> Parser {
        let lexer = Lexer::new(input);
        let mut parser = Parser {
            lexer,
            cur_token: Token::default(),
            local_vars: Environment::default(),
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

    fn create_ident(&mut self, ident: &Expression) -> PResult<Expression> {
        let Expression::Ident(ident) = ident else {
            return Err(ParserError::UnexpectedExpr(ident.clone()))
        };
        let name = ident.name.clone();
        let ident = Ident { name, offset: 0 };

        self.local_vars
            .register_item(ident.name.clone(), ident.clone());

        Ok(Expression::Ident(ident))
    }

    fn find_ident(&self, ident: &Expression) -> PResult<Expression> {
        let Expression::Ident(ident) = ident else {
            return Err(ParserError::UnexpectedExpr(ident.clone()))
        };
        let name = ident.name.clone();

        if let Some(ident) = self.local_vars.get_item(&name) {
            Ok(Expression::Ident(ident.clone()))
        } else {
            Err(ParserError::UndefinedIdent(name))
        }
    }

    fn enter_scope(&mut self) {
        self.local_vars.create_deeper_scope();
    }

    fn leave_scope(&mut self) {
        self.local_vars.create_shallow_scope();
    }

    /// program = stmt*
    pub fn parse_program(&mut self) -> PResult<Node<Expression>> {
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

        Ok(Node::Program(Program { statements }))
    }

    /// stmt = expr ';'?
    fn parse_stmt(&mut self) -> PResult<Statement<Expression>> {
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

    /// let_expr = 'let' (ident ':' type | ident '(' (ident ':' type),* ')' ':' type) '=' expr
    /// TODO: シャドーイングの実装
    fn parse_let_expr(&mut self) -> PResult<Expression> {
        self.expect(Let)?;

        let ident = self.parse_ident()?;

        if self.consume(LParen) {
            let name = Box::new(self.create_ident(&ident)?);
            let mut params = Vec::new();

            self.enter_scope();

            if self.consume(RParen) {
                let mut return_ty = self.parse_type()?;

                // 戻り値の型が明記されなかった場合は、戻り値がない関数とみなす
                if return_ty == Type::unknown() {
                    return_ty = Type::never();
                }

                self.expect(Assign)?;

                let body = Box::new(self.parse_expr()?);

                self.leave_scope();

                return Ok(Expression::FunctionDefExpr(FunctionDefExpr {
                    name,
                    params,
                    return_ty,
                    body,
                }));
            }

            let ident = self.parse_ident()?;
            let param = self.create_ident(&ident)?;
            let ty = self.parse_type()?;

            params.push((param, ty));

            while self.consume(Comma) {
                let ident = self.parse_ident()?;
                let param = self.create_ident(&ident)?;
                let ty = self.parse_type()?;

                params.push((param, ty));
            }

            self.expect(RParen)?;

            let mut return_ty = self.parse_type()?;

            // 戻り値の型が明記されなかった場合は、戻り値がない関数とみなす
            if return_ty == Type::unknown() {
                return_ty = Type::never();
            }

            self.expect(Assign)?;

            let body = Box::new(self.parse_expr()?);

            self.leave_scope();

            return Ok(Expression::FunctionDefExpr(FunctionDefExpr {
                name,
                params,
                return_ty,
                body,
            }));
        }

        let ty = self.parse_type()?;
        let name = Box::new(self.create_ident(&ident)?);

        self.expect(TokenKind::Assign)?;

        let value = Box::new(self.parse_expr()?);

        Ok(Expression::LetExpr(LetExpr { name, value, ty }))
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

        Ok(Expression::IfExpr(IfExpr {
            condition,
            consequence,
            alternative,
        }))
    }

    /// return_expr = 'return' expr
    fn parse_return_expr(&mut self) -> PResult<Expression> {
        self.expect(Return)?;

        Ok(Expression::ReturnExpr(ReturnExpr {
            value: Box::new(self.parse_expr()?),
        }))
    }

    /// assign = relation ('=' relation)?
    fn parse_assign(&mut self) -> PResult<Expression> {
        let node = self.parse_relation()?;

        if self.consume(TokenKind::Assign) {
            return Ok(Expression::BinaryExpr(BinaryExpr {
                kind: BinaryExprKind::Assign,
                lhs: Box::new(node),
                rhs: Box::new(self.parse_relation()?),
            }));
        }

        Ok(node)
    }

    /// relation = add ('==' add | '!=' add | '<' add | '<=' add | '>' add | '>=' add)*
    fn parse_relation(&mut self) -> PResult<Expression> {
        let mut node = self.parse_add()?;

        loop {
            if self.consume(TokenKind::Eq) {
                node = Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Eq,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });
            } else if self.consume(TokenKind::Ne) {
                node = Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Ne,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });
            } else if self.consume(TokenKind::Lt) {
                node = Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Lt,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });
            } else if self.consume(TokenKind::Le) {
                node = Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Le,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });
            } else if self.consume(TokenKind::Gt) {
                node = Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Gt,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });
            } else if self.consume(TokenKind::Ge) {
                node = Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Ge,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                });
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
                node = Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Add,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_mul()?),
                });
            } else if self.consume(Minus) {
                node = Expression::BinaryExpr(BinaryExpr {
                    kind: BinaryExprKind::Sub,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_mul()?),
                });
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
        self.enter_scope();

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

        self.leave_scope();

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
            offset: 0,
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

    /// type = ':' '*'* ident ('[' number ']')*
    fn parse_type(&mut self) -> PResult<Type> {
        if !self.consume(Colon) {
            return Ok(Type::unknown());
        }

        let mut pointer = 0;

        while self.consume(Asterisk) {
            pointer += 1;
        }

        let ident = self.expect(Ident)?.literal;
        let mut ty = Type::from(ident);

        for _ in 0..pointer {
            ty = Type::pointer_to(Box::new(ty));
        }

        while self.consume(LBracket) {
            let size = self
                .expect(Number)?
                .literal
                .parse()
                .map_err(ParserError::ParseIntError)?;

            ty = Type::array(Box::new(ty), size);
            self.expect(RBracket)?;
        }

        Ok(ty)
    }
}
