pub mod error;
#[cfg(test)]
mod test;

use self::error::ParserError;
use wervc_ast::{
    BinaryExprKind::*,
    Expr::{self, *},
    Node,
    Stmt::{self, *},
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

    /// expr = let_expr | add
    fn parse_expr(&mut self) -> PResult<Expr> {
        if self.peek(Let) {
            return self.parse_let_expr();
        }

        self.parse_assign()
    }

    /// let_expr = 'let' ident '=' expr
    fn parse_let_expr(&mut self) -> PResult<Expr> {
        self.expect(Let)?;

        let name = Box::new(self.parse_ident()?);

        self.expect(TokenKind::Assign)?;

        let value = Box::new(self.parse_expr()?);

        Ok(LetExpr { name, value })
    }

    /// assign = add ('=' add)?
    fn parse_assign(&mut self) -> PResult<Expr> {
        let node = self.parse_add()?;

        if self.consume(TokenKind::Assign) {
            return Ok(AssignExpr {
                name: Box::new(node),
                value: Box::new(self.parse_add()?),
            });
        }

        Ok(node)
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

    /// mul = call ('*' primary | '/' call)*
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

    /// call = primary ('(' expr,* ')')?
    fn parse_call(&mut self) -> PResult<Expr> {
        let node = self.parse_primary()?;

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

    /// primary = '(' expr ')' | block_expr | integer | ident
    fn parse_primary(&mut self) -> PResult<Expr> {
        if self.consume(LParen) {
            let expr = self.parse_expr()?;

            self.expect(RParen)?;

            return Ok(expr);
        }

        if self.peek(LBrace) {
            return self.parse_block_expr();
        }

        if self.peek(Number) {
            return self.parse_integer();
        }

        self.parse_ident()
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
}
