mod ast_printer;
mod interpreter;
mod lox;
mod parser;
mod scanner;
mod syntax;
mod token;
mod visit;

use std::env;
use std::path;

// use ast_printer::ASTStringVisitor;
// use syntax::BinaryExpr;
// use syntax::Expr;
// use syntax::Grouping;
// use syntax::UnaryExpr;
// use token::Token;

fn main() {
    let mut lox = lox::Lox::new();
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let _ = lox.run_prompt();
    } else if args.len() == 2 {
        let _ = lox.runfile(path::PathBuf::from(&args[1]));
    } else {
        println!("Usage: rlox [script]");
    }
}

// fn test_working_expressions() -> Expr {
//     Expr::Binary(BinaryExpr {
//         left: Box::new(Expr::Unary(UnaryExpr {
//             operator: Token::new(token::TokenType::Minus, 1),
//             right: Box::new(Expr::Literal(123.0.into())),
//         })),
//         operator: Token::new(token::TokenType::Star, 1),
//         right: Box::new(Expr::Grouping(Grouping {
//             expression: Box::new(Expr::Literal("WHAT".to_string().into())),
//         })),
//     })
// }
