#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Program(Vec<Statement>),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    LetStatement {
        name: Expression,
        value: Expression,
    },
    FunctionDefStatement {
        name: Expression,
        params: Vec<Expression>,
        body: Expression,
    },
    ReturnStatement(Expression),
    BlockReturnStatement(Expression),
    ExprStatement(Expression),
}
impl Into<Node> for Statement {
    fn into(self) -> Node {
        Node::Statement(self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    UnaryExpr {
        kind: UnaryExprKind,
        expr: Box<Expression>,
    },
    IfExpr {
        condition: Box<Expression>,
        consequence: Box<Expression>,
        alternative: Option<Box<Expression>>,
    },
    BlockExpr(Vec<Statement>),
    CallExpr {
        name: Box<Expression>,
        args: Vec<Expression>,
    },
    BinaryExpr {
        kind: BinaryExprKind,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Ident(String),
    Integer(isize),
    Boolean(bool),
}
impl Into<Node> for Expression {
    fn into(self) -> Node {
        Node::Expression(self)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryExprKind {
    Not,
    Minus,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Le,
    Lt,
    Ge,
    Gt,
}
