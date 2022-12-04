pub enum Node {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    LetStatement { name: Expression, value: Expression },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(String),
    Integer(isize),
}
