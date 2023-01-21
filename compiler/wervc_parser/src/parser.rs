pub mod error;
#[cfg(test)]
mod test;

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

use self::error::ParserError;

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

        while !self.consume(EOF) {
            let stmt = self.parse_stmt()?;

            stmts.push(stmt);
        }

        Ok(Node::Program(stmts))
    }

    /// stmt = expr ';'?
    pub fn parse_stmt(&mut self) -> PResult<Stmt> {
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

        self.parse_add()
    }

    /// let_expr = 'let' ident '=' expr
    fn parse_let_expr(&mut self) -> PResult<Expr> {
        self.expect(Let)?;

        let name = Box::new(self.parse_ident()?);

        self.expect(Assign)?;

        let value = Box::new(self.parse_expr()?);

        Ok(LetExpr { name, value })
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

    /// mul = primary ('*' primary | '/' primary)*
    fn parse_mul(&mut self) -> PResult<Expr> {
        let mut node = self.parse_primary()?;

        loop {
            if self.consume(Asterisk) {
                node = BinaryExpr {
                    kind: Mul,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_primary()?),
                };
            } else if self.consume(Slash) {
                node = BinaryExpr {
                    kind: Div,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_primary()?),
                };
            } else {
                return Ok(node);
            }
        }
    }

    /// primary = '(' expr ')' | integer | ident
    fn parse_primary(&mut self) -> PResult<Expr> {
        if self.consume(LParen) {
            let expr = self.parse_expr()?;

            self.expect(RParen)?;

            return Ok(expr);
        }

        if self.peek(Number) {
            return self.parse_integer();
        }

        self.parse_ident()
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
