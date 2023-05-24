pub mod ty;

use ty::Type;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program<E> {
    pub statements: Vec<Statement<E>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Node<E> {
    Program(Program<E>),
    Statement(Statement<E>),
    Expression(E),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Statement<E> {
    ExprStmt(E),
    ExprReturnStmt(E),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Integer {
    pub value: isize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Ident {
    pub name: String,
    pub offset: isize, // 型チェックの際にoffsetを計算する
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Boolean {
    pub value: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Array<E> {
    pub elements: Vec<E>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryExpr<E> {
    pub kind: BinaryExprKind,
    pub lhs: Box<E>,
    pub rhs: Box<E>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LetExpr<E> {
    pub name: Box<E>,
    pub value: Option<Box<E>>,
    pub ty: Type,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BlockExpr<E> {
    pub statements: Vec<Statement<E>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallExpr<E> {
    pub func: Box<E>,
    pub args: Vec<E>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionDefExpr<E> {
    pub name: Box<E>,
    // pair of (name, type)
    pub params: Vec<(E, Type)>,
    pub return_ty: Type,
    pub body: Box<E>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfExpr<E> {
    pub condition: Box<E>,
    pub consequence: Box<E>,
    pub alternative: Option<Box<E>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnExpr<E> {
    pub value: Box<E>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryExpr<E> {
    pub kind: UnaryExprKind,
    pub expr: Box<E>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Integer(Integer),
    Ident(Ident),
    Boolean(Boolean),
    Array(Array<Expression>),
    BinaryExpr(BinaryExpr<Expression>),
    LetExpr(LetExpr<Expression>),
    BlockExpr(BlockExpr<Expression>),
    CallExpr(CallExpr<Expression>),
    FunctionDefExpr(FunctionDefExpr<Expression>),
    IfExpr(IfExpr<Expression>),
    ReturnExpr(ReturnExpr<Expression>),
    UnaryExpr(UnaryExpr<Expression>),
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
