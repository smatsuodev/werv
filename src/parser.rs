mod ast;
#[cfg(test)]
mod test;

use self::ast::{ExprKind, Node};
use crate::lexer::{
    token::{Token, TokenKind},
    Lexer,
};
use std::vec::IntoIter;

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

    pub fn parse(&mut self) -> PResult {
        self.expr()
    }

    /// expr = add
    fn expr(&mut self) -> PResult {
        self.add()
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

    /// primary = '(' expr ')' | number
    fn primary(&mut self) -> PResult {
        if self.consume(TokenKind::LParen) {
            let node = self.expr()?;

            self.expect(TokenKind::RParen)?;

            return Ok(node);
        }

        let value = self.token_literal().parse::<isize>().or(Err(()))?;

        self.expect(TokenKind::Number)?;

        Ok(Box::new(Node::Integer(value)))
    }
}
