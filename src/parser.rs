mod error;
#[cfg(test)]
mod test;
use crate::{
    ast::{
        BinaryExprKind::*, Expression, Expression::*, Node, Statement, Statement::*,
        UnaryExprKind::*,
    },
    lexer::{error::LexerError, Lexer},
    token::{Token, TokenKind},
};

use self::error::{ParseError, ParseError::*};

type PResult<T> = Result<T, ParseError>;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}
impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            cur_token: Token::default(),
            peek_token: Token::default(),
        };

        p.next_token();
        p.next_token();
        p
    }

    pub fn parse(&mut self) -> PResult<Node> {
        let mut stmts = Vec::new();

        while !self.is_eof() {
            stmts.push(self.parse_statement()?);
        }

        Ok(Node::Program(stmts))
    }

    fn parse_statement(&mut self) -> PResult<Statement> {
        let stmt = self.parse_statement_body()?;

        self.consume(TokenKind::SemiColon)?;
        Ok(stmt)
    }

    fn parse_statement_body(&mut self) -> PResult<Statement> {
        match self.cur_token.kind() {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Fn => self.parse_fn_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_return_statement(&mut self) -> PResult<Statement> {
        self.consume(TokenKind::Return)?;

        let expr = self.parse_expression()?;

        Ok(ReturnStatement(expr))
    }

    fn parse_expr_statement(&mut self) -> PResult<Statement> {
        let expr = self.parse_expression()?;
        Ok(ExprStatement(expr))
    }

    fn parse_fn_statement(&mut self) -> PResult<Statement> {
        self.consume(TokenKind::Fn)?;

        let name = self.parse_ident()?;
        let params = self.parse_params()?;

        self.consume(TokenKind::Assign)?;

        let body = self.parse_expression()?;

        Ok(FunctionDefStatement { name, params, body })
    }

    fn parse_params(&mut self) -> PResult<Vec<Expression>> {
        self.consume(TokenKind::LParen)?;

        let mut params = Vec::new();

        if self.consume(TokenKind::RParen).is_ok() {
            return Ok(params);
        }

        params.push(self.parse_ident()?);

        while !self.consume(TokenKind::RParen).is_ok() {
            if self.is_eof() {
                return Err(ParseParamsError);
            }

            self.consume(TokenKind::Comma)?;
            params.push(self.parse_ident()?);
        }

        Ok(params)
    }

    fn parse_let_statement(&mut self) -> PResult<Statement> {
        self.consume(TokenKind::Let)?;

        let name = self.parse_ident()?;

        self.consume(TokenKind::Assign)?;

        let value = self.parse_expression()?;

        Ok(LetStatement { name, value })
    }

    fn parse_expression(&mut self) -> PResult<Expression> {
        if self.is_cur(TokenKind::If) {
            return self.parse_if();
        }

        if self.is_cur(TokenKind::LBrace) {
            return self.parse_block();
        }

        if self.is_cur(TokenKind::Bang)
            || self.is_cur(TokenKind::True)
            || self.is_cur(TokenKind::False)
        {
            return self.parse_bool();
        }

        self.parse_equal()
    }

    fn parse_if(&mut self) -> PResult<Expression> {
        self.consume(TokenKind::If)?;

        let condition = Box::new(self.parse_expression()?);
        let consequence = Box::new(self.parse_block()?);
        let alternative = if self.consume(TokenKind::Else).is_ok() {
            let expr = if self.is_cur(TokenKind::If) {
                self.parse_if()?
            } else {
                self.parse_block()?
            };

            Some(Box::new(expr))
        } else {
            None
        };

        Ok(IfExpr {
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_block(&mut self) -> PResult<Expression> {
        self.consume(TokenKind::LBrace)?;

        let mut stmts = Vec::new();

        while !self.consume(TokenKind::RBrace).is_ok() {
            let stmt_body = self.parse_statement_body()?;

            // もしセミコロンがない式で終わった場合
            if !self.is_cur(TokenKind::SemiColon) {
                if let Statement::ExprStatement(expr) = stmt_body {
                    stmts.push(BlockReturnStatement(expr));
                    self.consume(TokenKind::RBrace)?;
                    break;
                } else {
                    return Err(ParseBlockError);
                }
            }

            self.consume(TokenKind::SemiColon)?;
            stmts.push(stmt_body);
        }

        Ok(BlockExpr(stmts))
    }

    fn parse_bool(&mut self) -> PResult<Expression> {
        self.parse_bool_unary()
    }

    fn parse_bool_unary(&mut self) -> PResult<Expression> {
        if self.consume(TokenKind::Bang).is_ok() {
            return Ok(UnaryExpr {
                kind: Not,
                expr: Box::new(self.parse_bool_unary()?),
            });
        }

        self.parse_bool_primary()
    }

    fn parse_bool_primary(&mut self) -> PResult<Expression> {
        if self.consume(TokenKind::True).is_ok() {
            return Ok(Boolean(true));
        }

        if self.consume(TokenKind::False).is_ok() {
            return Ok(Boolean(false));
        }

        self.parse_primary()
    }

    fn parse_equal(&mut self) -> PResult<Expression> {
        let mut node = self.parse_relational()?;

        if self.consume(TokenKind::Eq).is_ok() {
            node = BinaryExpr {
                kind: Eq,
                lhs: Box::new(node),
                rhs: Box::new(self.parse_relational()?),
            };
        } else if self.consume(TokenKind::Ne).is_ok() {
            node = BinaryExpr {
                kind: Ne,
                lhs: Box::new(node),
                rhs: Box::new(self.parse_relational()?),
            };
        }

        Ok(node)
    }

    fn parse_relational(&mut self) -> PResult<Expression> {
        let mut node = self.parse_mod()?;

        if self.consume(TokenKind::Le).is_ok() {
            node = BinaryExpr {
                kind: Le,
                lhs: Box::new(node),
                rhs: Box::new(self.parse_mod()?),
            };
        } else if self.consume(TokenKind::Lt).is_ok() {
            node = BinaryExpr {
                kind: Lt,
                lhs: Box::new(node),
                rhs: Box::new(self.parse_mod()?),
            };
        } else if self.consume(TokenKind::Ge).is_ok() {
            node = BinaryExpr {
                kind: Ge,
                lhs: Box::new(node),
                rhs: Box::new(self.parse_mod()?),
            };
        } else if self.consume(TokenKind::Gt).is_ok() {
            node = BinaryExpr {
                kind: Gt,
                lhs: Box::new(node),
                rhs: Box::new(self.parse_mod()?),
            };
        }

        Ok(node)
    }

    fn parse_mod(&mut self) -> PResult<Expression> {
        let mut node = self.parse_add()?;

        loop {
            if self.consume(TokenKind::Percent).is_ok() {
                node = BinaryExpr {
                    kind: Mod,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_add()?),
                };
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_add(&mut self) -> PResult<Expression> {
        let mut node = self.parse_mul()?;

        loop {
            if self.consume(TokenKind::Plus).is_ok() {
                node = BinaryExpr {
                    kind: Add,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_mul()?),
                };
            } else if self.consume(TokenKind::Minus).is_ok() {
                node = BinaryExpr {
                    kind: Sub,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_mul()?),
                };
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_mul(&mut self) -> PResult<Expression> {
        let mut node = self.parse_unary()?;

        loop {
            if self.consume(TokenKind::Asterisk).is_ok() {
                node = BinaryExpr {
                    kind: Mul,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_unary()?),
                };
            } else if self.consume(TokenKind::Slash).is_ok() {
                node = BinaryExpr {
                    kind: Div,
                    lhs: Box::new(node),
                    rhs: Box::new(self.parse_unary()?),
                };
            } else {
                break;
            }
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> PResult<Expression> {
        if self.consume(TokenKind::Minus).is_ok() {
            return Ok(UnaryExpr {
                kind: Minus,
                expr: Box::new(self.parse_primary()?),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> PResult<Expression> {
        if self.is_cur(TokenKind::LParen) {
            return self.parse_paren_expr();
        }

        if self.is_cur(TokenKind::Ident) {
            if self.is_peek(TokenKind::LParen) {
                return self.parse_call_expression();
            }

            return self.parse_ident();
        }

        if self.is_cur(TokenKind::StringBody) {
            return self.parse_string();
        }

        return self.parse_integer();
    }

    fn parse_call_expression(&mut self) -> PResult<Expression> {
        let name = Box::new(self.parse_ident()?);
        let args = self.parse_args()?;

        Ok(CallExpr { name, args })
    }

    fn parse_args(&mut self) -> PResult<Vec<Expression>> {
        self.consume(TokenKind::LParen)?;

        let mut params = Vec::new();

        if self.consume(TokenKind::RParen).is_ok() {
            return Ok(params);
        }

        params.push(self.parse_expression()?);

        while !self.consume(TokenKind::RParen).is_ok() {
            if self.is_eof() {
                return Err(ParseArgsError);
            }

            self.consume(TokenKind::Comma)?;
            params.push(self.parse_expression()?);
        }

        Ok(params)
    }

    fn parse_paren_expr(&mut self) -> PResult<Expression> {
        self.consume(TokenKind::LParen)?;

        let expr = self.parse_expression()?;

        self.consume(TokenKind::RParen)?;
        return Ok(expr);
    }

    fn parse_string(&mut self) -> PResult<Expression> {
        let str = self.consume(TokenKind::StringBody)?;

        Ok(Str(str.literal()))
    }

    fn parse_integer(&mut self) -> PResult<Expression> {
        let token = self.consume(TokenKind::Number)?;
        let value = token
            .literal()
            .parse::<isize>()
            .or(Err(ParseIntegerError))?;

        Ok(Integer(value))
    }

    fn parse_ident(&mut self) -> PResult<Expression> {
        let token = self.consume(TokenKind::Ident)?;
        Ok(Ident(token.literal()))
    }

    fn is_eof(&self) -> bool {
        self.cur_token.kind() == TokenKind::EOF
    }

    fn next_token(&mut self) -> Option<LexerError> {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().ok()?;

        None
    }

    fn consume(&mut self, kind: TokenKind) -> PResult<Token> {
        if self.cur_token.kind() != kind {
            return Err(ParseConsumeError(kind));
        }

        let token = self.cur_token.clone();

        self.next_token()
            .map_or(Ok(()), |e| Err(ParseNextTokenError(e)))?;
        Ok(token)
    }

    fn is_cur(&self, kind: TokenKind) -> bool {
        self.cur_token.kind() == kind
    }

    fn is_peek(&self, kind: TokenKind) -> bool {
        self.peek_token.kind() == kind
    }
}
