mod ast;
#[cfg(test)]
mod test;

use self::ast::{ExprKind, Node};
use crate::lexer::{
    token::{Token, TokenKind},
    Lexer,
};

type PResult = Result<Box<Node>, ()>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl Parser<'_> {
    pub fn new(input: &str) -> Parser {
        Parser {
            lexer: Lexer::new(input),
        }
    }

    fn bump(&mut self) -> Token {
        self.lexer.next_token()
    }

    fn first(&self) -> Token {
        self.lexer.clone().next_token()
    }

    fn second(&self) -> Token {
        let mut lexer = self.lexer.clone();

        lexer.next_token();

        lexer.next_token()
    }

    fn is_eof(&self) -> bool {
        self.first().kind == TokenKind::EOF
    }

    fn token_literal(&self) -> &str {
        let mut lexer = self.lexer.clone();
        let len_remaining = lexer.chars.as_str().len();

        lexer.next_token();

        let len_eaten = len_remaining - lexer.chars.as_str().len();
        let whitespace_len = len_eaten - self.first().len;

        &self.lexer.chars.as_str()[whitespace_len..len_eaten]
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        if self.first().kind != kind {
            return false;
        }

        self.bump();

        true
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ()> {
        if self.first().kind != kind {
            return Err(());
        }

        Ok(self.bump())
    }

    fn expect_ident(&mut self) -> Result<String, ()> {
        let value = self.token_literal().to_string();

        self.expect(TokenKind::Ident)?;

        Ok(value)
    }

    fn eat_newlines(&mut self) {
        while self.consume(TokenKind::NewLine) {}
    }

    pub fn parse(&mut self) -> Result<Vec<Box<Node>>, ()> {
        let mut statements = Vec::new();

        while !self.is_eof() {
            self.eat_newlines();
            statements.push(self.statement()?);

            if !self.is_eof() {
                self.expect(TokenKind::NewLine)?;
            }
        }

        Ok(statements)
    }

    /// statement = assign | expr
    fn statement(&mut self) -> PResult {
        if self.second().kind == TokenKind::Assign {
            self.assign()
        } else {
            self.expr()
        }
    }

    /// assign = name '=' expr
    fn assign(&mut self) -> PResult {
        let name = Box::new(Node::Ident(self.expect_ident()?));

        self.expect(TokenKind::Assign)?;

        let expr = self.expr()?;

        Ok(Box::new(Node::Assign { name, expr }))
    }

    /// expr = equality
    fn expr(&mut self) -> PResult {
        self.equality()
    }

    /// equality = relational ( '==' relational | '!=' relational )*
    fn equality(&mut self) -> PResult {
        let mut node = self.relational()?;

        loop {
            if self.consume(TokenKind::Eq) {
                node = Box::new(Node::Expr {
                    kind: ExprKind::Eq,
                    lhs: node,
                    rhs: self.relational()?,
                })
            } else if self.consume(TokenKind::Ne) {
                node = Box::new(Node::Expr {
                    kind: ExprKind::Ne,
                    lhs: node,
                    rhs: self.relational()?,
                })
            } else {
                break;
            }
        }

        Ok(node)
    }

    /// relational = add ( '<' add | '<=' add | '>' add | '>=' add )
    fn relational(&mut self) -> PResult {
        let mut node = self.add()?;

        loop {
            if self.consume(TokenKind::Lt) {
                node = Box::new(Node::Expr {
                    kind: ExprKind::Lt,
                    lhs: node,
                    rhs: self.add()?,
                })
            } else if self.consume(TokenKind::Le) {
                node = Box::new(Node::Expr {
                    kind: ExprKind::Le,
                    lhs: node,
                    rhs: self.add()?,
                })
            } else if self.consume(TokenKind::Gt) {
                node = Box::new(Node::Expr {
                    kind: ExprKind::Gt,
                    lhs: node,
                    rhs: self.add()?,
                })
            } else if self.consume(TokenKind::Ge) {
                node = Box::new(Node::Expr {
                    kind: ExprKind::Ge,
                    lhs: node,
                    rhs: self.add()?,
                })
            } else {
                break;
            }
        }

        Ok(node)
    }

    /// add = mul ( '+' mul | '-' mul )*
    fn add(&mut self) -> PResult {
        let mut node = self.mul()?;

        loop {
            if self.consume(TokenKind::Plus) {
                node = Box::new(Node::Expr {
                    kind: ExprKind::Add,
                    lhs: node,
                    rhs: self.mul()?,
                })
            } else if self.consume(TokenKind::Minus) {
                node = Box::new(Node::Expr {
                    kind: ExprKind::Sub,
                    lhs: node,
                    rhs: self.mul()?,
                })
            } else {
                break;
            }
        }

        Ok(node)
    }

    /// mul = unary ( '*' unary | '/' unary )
    fn mul(&mut self) -> PResult {
        let mut node = self.unary()?;

        loop {
            if self.consume(TokenKind::Asterisk) {
                node = Box::new(Node::Expr {
                    kind: ExprKind::Mul,
                    lhs: node,
                    rhs: self.unary()?,
                })
            } else if self.consume(TokenKind::Slash) {
                node = Box::new(Node::Expr {
                    kind: ExprKind::Div,
                    lhs: node,
                    rhs: self.unary()?,
                })
            } else {
                break;
            }
        }

        Ok(node)
    }

    /// unary = '-'? primary
    fn unary(&mut self) -> PResult {
        if self.consume(TokenKind::Minus) {
            Ok(Box::new(Node::Expr {
                kind: ExprKind::Sub,
                lhs: Box::new(Node::Integer(0)),
                rhs: self.primary()?,
            }))
        } else {
            self.primary()
        }
    }

    /// primary = '(' expr ')' | number | ident
    fn primary(&mut self) -> PResult {
        if self.consume(TokenKind::LParen) {
            let node = self.expr()?;

            self.expect(TokenKind::RParen)?;

            return Ok(node);
        }

        let literal = self.token_literal().to_string();

        if self.consume(TokenKind::Number) {
            let value = literal.parse::<isize>().or(Err(()))?;

            return Ok(Box::new(Node::Integer(value)));
        }

        self.expect(TokenKind::Ident)?;

        return Ok(Box::new(Node::Ident(literal.to_string())));
    }
}
