use std::fmt;
use std::iter::Peekable;

use crate::{
    ast_printer::ASTStringVisitor,
    syntax::{BinaryExpr, Expr, Grouping, LiteralValue, UnaryExpr},
    token::{Token, TokenType as TT},
};

type BoxIterToken = Box<dyn Iterator<Item = Token>>;
type TokenPeekable = Peekable<BoxIterToken>;
type ExprResult = Result<Expr, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    UnterminatedParentheses(usize, usize),
    NonPrimaryToken(Token),
    EmptyPrimary(usize),
    EmptyExpression(usize),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::UnterminatedParentheses(r_line, l_line) => {
                write!(f, "Parser Error: Expecting terminating parentheses at line {}, unterminated parentheses located at line {}", r_line, l_line)?;
            }
            ParserError::NonPrimaryToken(token) => {
                write!(
                    f,
                    "Parser Error: Unsupported token {:?} at line {}",
                    token.token_type, token.line
                )?;
            }
            ParserError::EmptyPrimary(line) => {
                write!(
                    f,
                    "Parser Error: Expecting a token here at line {}, none found.",
                    line
                )?;
            }
            ParserError::EmptyExpression(line) => {
                write!(
                    f,
                    "Parser Error: Empty expressions are illegal, found at line {}",
                    line
                )?;
            }
        }

        Ok(())
    }
}

impl ParserError {
    pub fn line(&self) -> usize {
        match *self {
            ParserError::UnterminatedParentheses(line, _) => line,
            ParserError::NonPrimaryToken(Token { line, .. }) => line,
            ParserError::EmptyPrimary(line) => line,
            ParserError::EmptyExpression(line) => line,
        }
    }
}

pub struct Parser {
    tokens: TokenPeekable,
    prev_token_line: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let iter_tokens: BoxIterToken = Box::new(tokens.into_iter());
        Parser {
            tokens: iter_tokens.peekable(),
            prev_token_line: 0,
        }
    }

    pub fn parse(&mut self) -> ExprResult {
        self.expression()
    }

    fn binary_expr_generator(
        &mut self,
        expr_fn: Box<dyn Fn(&mut Parser) -> ExprResult>,
        expr_tokens: &[TT],
    ) -> ExprResult {
        let mut expr = expr_fn(self)?;

        while let Some(token) = self.tokens.peek() {
            if expr_tokens.contains(&token.token_type) {
                // Update prev token line pointer
                self.prev_token_line = token.line;

                let operator = self.tokens.next().unwrap();
                let right = expr_fn(self)?;
                expr = Expr::Binary(BinaryExpr {
                    left: Box::new(expr),
                    right: Box::new(right),
                    operator,
                })
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn expression(&mut self) -> ExprResult {
        return self.equality();
    }

    fn equality(&mut self) -> ExprResult {
        let equality_tokens = [TT::BangEqual, TT::EqualEqual];
        self.binary_expr_generator(Box::new(Parser::comparison), &equality_tokens)
    }

    fn comparison(&mut self) -> ExprResult {
        let comparison_tokens = [TT::Greater, TT::GreaterEqual, TT::Less, TT::LessEqual];
        self.binary_expr_generator(Box::new(Parser::term), &comparison_tokens)
    }

    fn term(&mut self) -> ExprResult {
        let term_tokens = [TT::Minus, TT::Plus];
        self.binary_expr_generator(Box::new(Parser::factor), &term_tokens)
    }

    fn factor(&mut self) -> ExprResult {
        let factor_tokens = [TT::Slash, TT::Star];
        self.binary_expr_generator(Box::new(Parser::unary), &factor_tokens)
    }

    fn unary(&mut self) -> ExprResult {
        let unary_tokens = [TT::Bang, TT::Minus];
        let intrem = match self.tokens.peek() {
            Some(token) if unary_tokens.contains(&token.token_type) => {
                // Update prev token line pointer
                self.prev_token_line = token.line;

                let operator = self.tokens.next().unwrap();
                let right = self.unary()?;
                Ok(Expr::Unary(UnaryExpr {
                    right: Box::new(right),
                    operator,
                }))
            }

            _ => self.primary(),
        };

        intrem
    }

    fn primary(&mut self) -> ExprResult {
        if let Some(peek_token) = self.tokens.peek() {
            // Update prev token line pointer
            self.prev_token_line = peek_token.line;

            match &peek_token.token_type {
                // Handle parentheses scoping
                TT::LeftParen => {
                    let token = self.tokens.next().unwrap();
                    let result_expr = self.expression();

                    let expr = match result_expr {
                        Ok(ex) => Ok(ex),

                        Err(ParserError::NonPrimaryToken(token)) => match token.token_type {
                            TT::RightParen => Err(ParserError::EmptyExpression(token.line)),
                            _ => Err(ParserError::NonPrimaryToken(token)),
                        },
                        Err(err) => Err(err),
                    }?;

                    match self
                        .tokens
                        .next()
                        .ok_or(ParserError::UnterminatedParentheses(
                            self.prev_token_line,
                            token.line,
                        ))?
                        .token_type
                    {
                        TT::RightParen => Ok(Expr::Grouping(Grouping {
                            expression: Box::new(expr),
                        })),
                        _token => Err(ParserError::UnterminatedParentheses(
                            self.prev_token_line,
                            token.line,
                        )),
                    }
                }

                // Handle literals
                TT::False => self.consume_and_cast_literal(false.into()),
                TT::True => self.consume_and_cast_literal(true.into()),
                TT::Nil => self.consume_and_cast_literal(LiteralValue::None),
                TT::Number(borrowed_float) => {
                    let float = borrowed_float.clone();
                    self.consume_and_cast_literal(float.into())
                }
                TT::LoxString(borrowed_str) => {
                    let lox_string = borrowed_str.clone();
                    self.consume_and_cast_literal(lox_string.into())
                }

                _ => Err(ParserError::NonPrimaryToken(peek_token.clone())),
            }
        } else {
            Err(ParserError::EmptyPrimary(self.prev_token_line))
        }
    }

    fn consume_and_cast_literal(&mut self, literal_value: LiteralValue) -> ExprResult {
        self.tokens.next();
        Ok(Expr::Literal(literal_value))
    }

    fn synchronize(&mut self) {
        self.tokens.next();

        while let Some(token) = self.tokens.peek() {
            if [
                TT::Class,
                TT::Fun,
                TT::Var,
                TT::For,
                TT::If,
                TT::While,
                TT::Print,
                TT::Return,
            ]
            .contains(&token.token_type)
            {
                return;
            }

            self.tokens.next();
        }
    }
}
