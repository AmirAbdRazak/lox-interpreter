use crate::syntax::{BinaryExpr, Expr, Grouping, UnaryExpr};
use crate::visit::MutVisitor;

pub struct Interpreter {}

impl MutVisitor for Interpreter {
    type E = Result<Types, String>;

    fn visit_expression(&mut self, expr: &Expr) -> Self::E {
        match expr {
            Expr::Binary(BinaryExpr {
                left: left_expr,
                operator,
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
            Expr::Literal(lit) => {}
            Expr::Grouping(Grouping { expression }) => {
                let expr = self.visit_expression(expression)?;
            }

            _ => unreachable!(),
        }

        return Err(String::from("error"));
    }
}

pub enum Types {
    Number(f64),
}
