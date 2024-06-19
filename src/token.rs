use std::fmt;

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
    Identifier(String),
    LoxString(String),
    Number(f64),
    Nil,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Token {
        Token { token_type, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TokenType: {:?} at Line {}", self.token_type, self.line)?;

        Ok(())
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use TokenType::*;
        let value: &str = match self {
            LeftParen => "(",
            RightParen => ")",
            LeftBrace => "{",
            RightBrace => "}",
            Comma => ",",
            Dot => ".",
            Minus => "-",
            Plus => "+",
            Semicolon => ";",
            Slash => "/",
            Star => "*",

            // One or two character tokens.
            Bang => "!",
            BangEqual => "!=",
            Equal => "=",
            EqualEqual => "==",
            Greater => ">",
            GreaterEqual => "=>",
            Less => "<",
            LessEqual => "<=",

            // Literals.
            Identifier(identifier) => identifier,
            LoxString(literal) => literal,
            Number(f64) => &f64.to_string(),
            Nil => "Nil",

            // Keywords.
            And => "AND",
            Class => "CLASS",
            Else => "ELSE",
            False => "FALSE",
            Fun => "FUN",
            For => "FOR",
            If => "IF",
            Or => "OR",
            Print => "PRINT",
            Return => "RETURN",
            Super => "SUPER",
            This => "THIS",
            True => "TRUE",
            Var => "VAR",
            While => "WHILE",

            Eof => "EOF",
        };

        write!(f, "{}", value)?;

        Ok(())
    }
}
