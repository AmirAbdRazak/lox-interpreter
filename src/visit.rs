use crate::syntax::Expr;

pub trait MutVisitor {
    type E;

    fn visit_expression(&mut self, expr: &Expr) -> Self::E;
}

pub trait Visitor {
    type E;

    fn visit_expression(&self, expr: &Expr) -> Self::E;
}
