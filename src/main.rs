mod ast_printer;
mod interpreter;
mod lox;
mod scanner;
mod syntax;
mod token;
mod visit;

use std::env;
use std::path;

use ast_printer::ASTStringVisitor;
use syntax::BinaryExpr;
use syntax::Expr;
use syntax::Grouping;
use syntax::UnaryExpr;
use token::Literal;
use token::Token;

fn main() {
    let mut lox = lox::Lox::new();
    let args: Vec<String> = env::args().collect();

    let expression = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Token::new(token::TokenType::Minus, 1, Some("-".to_string()), None),
            right: Box::new(Expr::Literal(Literal::Float(123.0))),
        })),
        operator: Token::new(token::TokenType::Star, 1, Some("*".to_string()), None),
        right: Box::new(Expr::Grouping(Grouping {
            expression: Box::new(Expr::Literal(Literal::Str("string literal".to_string()))),
        })),
    });

    let sv = ASTStringVisitor {
        expressions: &[expression],
    };

    println!("{}", sv);

    if args.len() == 1 {
        let _ = lox.run_prompt();
    } else if args.len() == 2 {
        let _ = lox.runfile(path::PathBuf::from(&args[1]));
    } else {
        println!("Usage: rlox [script]");
    }
}
