use crate::{ast_printer::ASTStringVisitor, token::Token};
use std::fmt;

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Bang,
    Minus,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}
#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Float(f64),
    LoxString(String),
    Bool(bool),
    None,
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            LiteralValue::Float(float) => float.to_string(),
            LiteralValue::None => "None".to_string(),
            LiteralValue::Bool(bool) => bool.to_string(),
            LiteralValue::LoxString(literal) => format!("\"{}\"", literal),
        };

        write!(f, "{}", value)?;

        Ok(())
    }
}
impl From<f64> for LiteralValue {
    fn from(value: f64) -> Self {
        LiteralValue::Float(value)
    }
}

impl From<String> for LiteralValue {
    fn from(value: String) -> Self {
        LiteralValue::LoxString(value)
    }
}

impl From<bool> for LiteralValue {
    fn from(value: bool) -> Self {
        LiteralValue::Bool(value)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(Grouping),
    Literal(LiteralValue),
    Unary(UnaryExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let expr_str = ASTStringVisitor {
            expressions: &[self.clone()],
        };
        write!(f, "{}", expr_str)?;

        Ok(())
    }
}
