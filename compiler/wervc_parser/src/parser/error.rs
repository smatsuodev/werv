use std::num::ParseIntError;
use wervc_ast::Expression;
use wervc_lexer::token::TokenKind;

#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
    UnexpectedToken {
        expected: TokenKind,
        actual: TokenKind,
    },
    ParseIntError(ParseIntError),
    RequiredSemiColon,
    IdentAlreadyDefined(String),
    UndefinedIdent(String),
    U,
    UnexpectedExpr(Expression),
}
