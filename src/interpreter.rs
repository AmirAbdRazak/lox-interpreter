use crate::syntax::{BinaryExpr, Expr, UnaryExpr};
use crate::visit::MutVisitor;

pub struct Interpreter {}

impl MutVisitor for Interpreter {
    type E = Result<Types, String>;

    fn visit_expression(&mut self, expr: &Expr) -> Self::E {
        match expr {
            Expr::Binary(BinaryExpr {
                left: left_expr,
                operator: operator,
                right: right_expr,
            }) => {
                let right = self.visit_expression(right_expr)?;
                let left = self.visit_expression(left_expr)?;
            }
            Expr::Unary(UnaryExpr {
                operator,
                right: right_expr,
            }) => {
                let right = self.visit_expression(right_expr)?;
            }

            _ => unreachable!(),
        }

        return Err(String::from("error"));
    }
}

pub enum Types {
    Number(f64),
}
