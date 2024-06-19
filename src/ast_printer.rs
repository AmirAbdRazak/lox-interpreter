use std::fmt;

use crate::{
    syntax::{BinaryExpr, Expr, Grouping, UnaryExpr},
    visit::Visitor,
};

pub struct ASTStringVisitor<'a> {
    pub expressions: &'a [Expr],
}

impl<'a> Visitor for ASTStringVisitor<'a> {
    type E = String;

    fn visit_expression(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(BinaryExpr {
                left: left_expr,
                right: right_expr,
                operator,
            }) => format!(
                "(Binary {} {} {})",
                operator.token_type,
                self.visit_expression(left_expr),
                self.visit_expression(right_expr)
            ),
            Expr::Unary(UnaryExpr {
                operator,
                right: right_expr,
            }) => format!(
                "(Unary {} {})",
                operator.token_type,
                self.visit_expression(right_expr)
            ),
            Expr::Grouping(Grouping { expression: expr }) => {
                format!("(Grouping {})", self.visit_expression(expr))
            }
            Expr::Literal(literal_value) => format!("(Literal {})", literal_value),
        }
    }
}

impl<'a> fmt::Display for ASTStringVisitor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for expr in self.expressions {
            write!(f, "{}", self.visit_expression(expr))?;
        }

        Ok(())
    }
}
