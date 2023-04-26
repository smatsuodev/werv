#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub total_offset: isize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement {
    ExprStmt(Expression),
    ExprReturnStmt(Expression),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Integer {
    pub value: isize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ident {
    pub name: String,
    pub offset: isize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Boolean {
    pub value: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Array {
    pub elements: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryExpr {
    pub kind: BinaryExprKind,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LetExpr {
    pub name: Box<Expression>,
    pub value: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BlockExpr {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallExpr {
    pub func: Box<Expression>,
    pub args: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionDefExpr {
    pub name: Box<Expression>,
    pub params: Vec<Expression>,
    pub body: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfExpr {
    pub condition: Box<Expression>,
    pub consequence: Box<Expression>,
    pub alternative: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnExpr {
    pub value: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryExpr {
    pub kind: UnaryExprKind,
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IndexExpr {
    pub array: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Integer(Integer),
    Ident(Ident),
    Boolean(Boolean),
    Array(Array),
    BinaryExpr(BinaryExpr),
    LetExpr(LetExpr),
    BlockExpr(BlockExpr),
    CallExpr(CallExpr),
    FunctionDefExpr(FunctionDefExpr),
    IfExpr(IfExpr),
    ReturnExpr(ReturnExpr),
    UnaryExpr(UnaryExpr),
    IndexExpr(IndexExpr),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Assign,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnaryExprKind {
    Minus,
    Not,
    Deref,
    Addr,
}
