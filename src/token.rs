use core::fkt;
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Float(f64),
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub lexeme: Option<String>,
    pub literal: Option<Literal>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line: usize,
        lexeme: Option<String>,
        literal: Option<Literal>,
    ) -> Token {
        Token {
            token_type,
            line,
            lexeme,
            literal,
        }
    }
}
