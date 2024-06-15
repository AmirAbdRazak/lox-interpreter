use crate::token::{Literal, Token};

pub enum UnaryOperator {
    Bang,
    Minus,
}

pub enum BinaryOperator {
    Minus,
    Plus,
    Slash,
    Star,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
pub struct Grouping {
    pub expression: Box<Expr>,
}
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(Grouping),
    Literal(Literal),
    Unary(UnaryExpr),
}
