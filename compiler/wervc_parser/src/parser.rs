pub mod error;
#[cfg(test)]
mod test;

use self::error::ParserError;
use wervc_ast::{
    BinaryExprKind::{self, *},
    Expr::{self, *},
    Node,
    Stmt::{self, *},
    UnaryExprKind,
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
}

type PResult<T> = Result<T, ParserError>;

impl Parser {
    pub fn new(input: impl ToString) -> Parser {
        let lexer = Lexer::new(input);
        let mut parser = Parser {
            lexer,
            cur_token: Token::default(),
        };

        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.lexer.next_token()
    }

    fn peek(&self, kind: TokenKind) -> bool {
        self.cur_token.kind() == kind
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        if self.cur_token.kind() == kind {
            self.next_token();
            return true;
        }

        false
    }

    fn expect(&mut self, kind: TokenKind) -> PResult<Token> {
        if self.cur_token.kind() != kind {
            return Err(ParserError::UnexpectedToken {
                expected: kind,
                got: self.cur_token.kind(),
            });
        }

        let token = self.cur_token.clone();

        self.next_token();

        Ok(token)
    }

    /// program = stmt*
    pub fn parse_program(&mut self) -> PResult<Node> {
        let mut stmts = Vec::new();
        let mut is_returned = false;

        while !self.consume(EOF) {
            let stmt = self.parse_stmt()?;

            if is_returned {
                return Err(ParserError::RequiredSemiColon);
            }

            is_returned = matches!(stmt, ExprReturnStmt(_));
            stmts.push(stmt);
        }

        Ok(Node::Program(stmts))
    }

    /// stmt = expr ';'?
    fn parse_stmt(&mut self) -> PResult<Stmt> {
        let expr = self.parse_expr()?;

        if self.consume(SemiColon) {
            return Ok(ExprStmt(expr));
        }

        Ok(ExprReturnStmt(expr))
    }

    /// expr = let_expr | if_expr | return_expr | assign
    fn parse_expr(&mut self) -> PResult<Expr> {
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

    /// let_expr = 'let' (ident | ident '(' ident,* ')') '=' expr
    fn parse_let_expr(&mut self) -> PResult<Expr> {
        self.expect(Let)?;

        let name = Box::new(self.parse_ident()?);

        if self.consume(LParen) {
            let mut params = Vec::new();

            if self.consume(RParen) {
                self.expect(Assign)?;

                let body = Box::new(self.parse_expr()?);

                return Ok(FunctionDefExpr { name, params, body });
            }

            let token = self.parse_ident()?;

            params.push(token);

            while self.consume(Comma) {
                let token = self.parse_ident()?;

                params.push(token);
            }

            self.expect(RParen)?;
            self.expect(Assign)?;

            let body = Box::new(self.parse_expr()?);

            return Ok(FunctionDefExpr { name, params, body });
        }

        self.expect(TokenKind::Assign)?;

        let value = Box::new(self.parse_expr()?);

        Ok(LetExpr { name, value })
    }

    /// if_expr = 'if' expr expr ('else' expr)?
    fn parse_if_expr(&mut self) -> PResult<Expr> {
        self.expect(If)?;

        let condition = Box::new(self.parse_expr()?);
        let consequence = Box::new(self.parse_expr()?);
        let alternative = if self.consume(Else) {
            Some(Box::new(self.parse_expr()?))
        } else {
            None
        };

        Ok(IfExpr {
            condition,
            consequence,
            alternative,
        })
    }

    /// return_expr = 'return' expr
    fn parse_return_expr(&mut self) -> PResult<Expr> {
        self.expect(Return)?;

        Ok(ReturnExpr(Box::new(self.parse_expr()?)))
    }

    /// assign = add ('=' add)?
    fn parse_assign(&mut self) -> PResult<Expr> {
        let node = self.parse_relation()?;

        if self.consume(TokenKind::Assign) {
            return Ok(AssignExpr {
                name: Box::new(node),
                value: Box::new(self.parse_relation()?),
            });
        }

        Ok(node)
    }

    /// relation = add ('==' add | '!=' add | '<' add | '<=' add | '>' add | '>=' add)*
    fn parse_relation(&mut self) -> PResult<Expr> {
        let mut node = self.parse_add()?;

        loop {
            if self.consume(TokenKind::Eq) {
                node = BinaryExpr {
                    kind: BinaryExprKind::Eq,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                };
            } else if self.consume(TokenKind::Ne) {
                node = BinaryExpr {
                    kind: BinaryExprKind::Ne,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                };
            } else if self.consume(TokenKind::Lt) {
                node = BinaryExpr {
                    kind: BinaryExprKind::Lt,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                };
            } else if self.consume(TokenKind::Le) {
                node = BinaryExpr {
                    kind: BinaryExprKind::Le,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                };
            } else if self.consume(TokenKind::Gt) {
                node = BinaryExpr {
                    kind: BinaryExprKind::Gt,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                };
            } else if self.consume(TokenKind::Ge) {
                node = BinaryExpr {
                    kind: BinaryExprKind::Ge,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                };
            } else {
                return Ok(node);
            }
        }
    }

    /// add = mul ('+' mul | '-' mul)*
    fn parse_add(&mut self) -> PResult<Expr> {
        let mut node = self.parse_mul()?;

        loop {
            if self.consume(Plus) {
                node = BinaryExpr {
                    kind: Add,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_mul()?),
                };
            } else if self.consume(Minus) {
                node = BinaryExpr {
                    kind: Sub,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_mul()?),
                };
            } else {
                return Ok(node);
            }
        }
    }

    /// mul = call ('*' call | '/' call)*
    fn parse_mul(&mut self) -> PResult<Expr> {
        let mut node = self.parse_call()?;

        loop {
            if self.consume(Asterisk) {
                node = BinaryExpr {
                    kind: Mul,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_call()?),
                };
            } else if self.consume(Slash) {
                node = BinaryExpr {
                    kind: Div,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_call()?),
                };
            } else {
                return Ok(node);
            }
        }
    }

    /// call = unary ('(' expr,* ')')?
    fn parse_call(&mut self) -> PResult<Expr> {
        let node = self.parse_unary()?;

        if self.consume(LParen) {
            let mut args = Vec::new();

            if self.consume(RParen) {
                return Ok(CallExpr {
                    func: Box::new(node),
                    args,
                });
            }

            args.push(self.parse_expr()?);

            while self.consume(Comma) {
                args.push(self.parse_expr()?);
            }

            self.expect(RParen)?;

            return Ok(CallExpr {
                func: Box::new(node),
                args,
            });
        }

        Ok(node)
    }

    /// unary = '!' unary | '-' primary | primary
    fn parse_unary(&mut self) -> PResult<Expr> {
        if self.consume(Bang) {
            return Ok(UnaryExpr {
                kind: UnaryExprKind::Not,
                expr: Box::new(self.parse_unary()?),
            });
        }

        if self.consume(Minus) {
            return Ok(UnaryExpr {
                kind: UnaryExprKind::Minus,
                expr: Box::new(self.parse_primary()?),
            });
        }

        self.parse_primary()
    }

    /// primary = '(' expr ')' | block_expr | array | integer | ident | bool
    fn parse_primary(&mut self) -> PResult<Expr> {
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
            return self.parse_ident();
        }

        self.parse_bool()
    }

    /// block_expr = '{' stmt* '}'
    fn parse_block_expr(&mut self) -> PResult<Expr> {
        self.expect(LBrace)?;

        let mut stmts = Vec::new();

        while !self.consume(RBrace) {
            if self.consume(EOF) {
                return Err(ParserError::UnexpectedToken {
                    expected: RBrace,
                    got: EOF,
                });
            }

            let stmt = self.parse_stmt()?;

            stmts.push(stmt);
        }

        Ok(BlockExpr(stmts))
    }

    /// array = '[' expr,* ']'
    fn parse_array(&mut self) -> PResult<Expr> {
        self.expect(LBracket)?;

        let mut values = Vec::new();

        if self.consume(RBracket) {
            return Ok(Array(values));
        }

        values.push(self.parse_expr()?);

        while self.consume(Comma) {
            values.push(self.parse_expr()?);
        }

        self.expect(RBracket)?;

        Ok(Array(values))
    }

    /// integer = [0-9]*
    fn parse_integer(&mut self) -> PResult<Expr> {
        let token = self.expect(Number)?;
        let literal = token
            .literal()
            .parse::<isize>()
            .map_err(ParserError::ParseIntError)?;

        Ok(Integer(literal))
    }

    /// ident = ([a-zA-Z] | '_') ([a-zA-Z0-9] | '_')*
    fn parse_ident(&mut self) -> PResult<Expr> {
        let token = self.expect(TokenKind::Ident)?;
        let literal = token.literal().to_string();

        Ok(Expr::Ident(literal))
    }

    /// bool = 'true' | 'false'
    fn parse_bool(&mut self) -> PResult<Expr> {
        if self.consume(True) {
            return Ok(Boolean(true));
        }

        self.expect(False)?;

        Ok(Boolean(false))
    }
}
