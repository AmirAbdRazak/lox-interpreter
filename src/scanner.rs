use crate::token::{Token, TokenType};
use std::iter;
use std::str;
use std::{collections, fmt};

pub type ScannerResult<T> = Result<T, ScannerError>;

#[derive(Debug)]
pub enum ScannerError {
    UnknownCharacter(char, usize),
    UnterminatedString(usize),
    UnparseableDigit(String, usize),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScannerError::UnknownCharacter(c, line) => {
                write!(
                    f,
                    "Scanner Error: Unrecognised character {} at line {}",
                    c, line
                )?;
            }
            ScannerError::UnterminatedString(line) => {
                write!(f, "Scanner Error: Unterminated string at line {}", line)?;
            }
            ScannerError::UnparseableDigit(err_str, line) => {
                write!(
                    f,
                    "Scanner Error: Unparseable digit {} at line {}",
                    err_str, line
                )?;
            }
        }

        Ok(())
    }
}

impl ScannerError {
    pub fn line(&self) -> usize {
        match *self {
            ScannerError::UnknownCharacter(_, line) => line,
            ScannerError::UnterminatedString(line) => line,
            ScannerError::UnparseableDigit(_, line) => line,
        }
    }
}

pub struct Scanner<'a> {
    source: iter::Peekable<str::Chars<'a>>,
    line: usize,
    keywords: collections::HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.chars().peekable(),
            line: 1,
            keywords: Self::get_keywords(),
        }
    }

    pub fn get_keywords() -> collections::HashMap<String, TokenType> {
        let mut keywords = collections::HashMap::new();

        use crate::token::TokenType::*;
        keywords.insert("or".to_string(), Or);
        keywords.insert("and".to_string(), And);
        keywords.insert("if".to_string(), If);
        keywords.insert("else".to_string(), Else);
        keywords.insert("var".to_string(), Var);
        keywords.insert("for".to_string(), For);
        keywords.insert("while".to_string(), While);
        keywords.insert("fun".to_string(), Fun);
        keywords.insert("class".to_string(), Class);
        keywords.insert("super".to_string(), Super);
        keywords.insert("this".to_string(), This);
        keywords.insert("return".to_string(), Return);
        keywords.insert("true".to_string(), True);
        keywords.insert("false".to_string(), False);
        keywords.insert("nil".to_string(), Nil);

        keywords
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScannerError>> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<ScannerError> = Vec::new();

        loop {
            self.skip_whitespace();
            let ch = self.source.next();
            match ch {
                Some(ch) => {
                    if !self.skip_comments(ch) {
                        match self.scan_token(ch) {
                            Ok(token) => tokens.push(token),
                            Err(err) => errors.push(err),
                        }
                    }
                }
                None => {
                    tokens.push(self.simple_token(TokenType::Eof));
                    break;
                }
            };
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(tokens)
    }

    pub fn simple_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
        }
    }

    pub fn scan_operator(
        &mut self,
        token_type: TokenType,
        equality_token_type: TokenType,
    ) -> Token {
        if self.source.peek() == Some(&'=') {
            // Consume the =
            self.source.next();
            self.simple_token(equality_token_type)
        } else {
            self.simple_token(token_type)
        }
    }

    pub fn skip_comments(&mut self, ch: char) -> bool {
        if ch == '/' && self.source.peek() == Some(&'/') {
            while let Some(&c) = self.source.peek() {
                self.source.next();
                if c == '\n' {
                    self.line += 1;
                    return true;
                }
            }
        }

        return false;
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(&c) = self.source.peek() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1
                }

                self.source.next();
            } else {
                return;
            }
        }
    }

    pub fn parse_string(&mut self) -> ScannerResult<Token> {
        let mut string = String::new();
        let line = self.line;

        while let Some(&c) = self.source.peek() {
            if c == '"' {
                self.source.next();
                return Ok(self.simple_token(TokenType::LoxString(string)));
            } else if c == '\n' {
                self.line += 1;
            }
            string.push(self.source.next().unwrap());
        }

        return Err(ScannerError::UnterminatedString(line));
    }

    pub fn parse_number(&mut self, ch: char) -> ScannerResult<Token> {
        let mut string = String::new();
        string.push(ch);

        while let Some(&c) = self.source.peek() {
            if c == '.' {
                string.push(c);
            } else if c.is_alphanumeric() {
                string.push(c);
            } else if c.is_whitespace() {
                break;
            }
            self.source.next();
        }

        match string.parse() {
            Ok(float) => Ok(self.simple_token(TokenType::Number(float))),
            Err(_) => {
                return Err(ScannerError::UnparseableDigit(string, self.line));
            }
        }
    }

    pub fn parse_identifier(&mut self, ch: char) -> ScannerResult<Token> {
        let mut string = String::new();
        string.push(ch);

        while let Some(&c) = self.source.peek() {
            if !c.is_alphabetic() && c != '_' && !c.is_digit(10) {
                break;
            }
            string.push(c);
            self.source.next();
        }

        match self.keywords.get(&string) {
            Some(keyword_type) => Ok(self.simple_token(keyword_type.clone())),
            None => Ok(self.simple_token(TokenType::Identifier(string))),
        }
    }

    pub fn scan_token(&mut self, ch: char) -> ScannerResult<Token> {
        use crate::token::TokenType::*;
        let token = match ch {
            '(' => self.simple_token(LeftParen),
            ')' => self.simple_token(RightParen),
            '{' => self.simple_token(LeftBrace),
            '}' => self.simple_token(RightBrace),
            ',' => self.simple_token(Comma),
            '.' => self.simple_token(Dot),
            '-' => self.simple_token(Minus),
            '+' => self.simple_token(Plus),
            ';' => self.simple_token(Semicolon),
            '*' => self.simple_token(Star),

            '!' => self.scan_operator(Bang, BangEqual),
            '=' => self.scan_operator(Equal, EqualEqual),
            '<' => self.scan_operator(Less, LessEqual),
            '>' => self.scan_operator(Greater, GreaterEqual),

            '/' => self.simple_token(Slash),

            '"' => return self.parse_string(),

            ch @ _ => {
                if ch.is_digit(10) {
                    return self.parse_number(ch);
                } else if ch.is_alphabetic() || ch == '_' {
                    return self.parse_identifier(ch);
                } else {
                    return Err(ScannerError::UnknownCharacter(ch, self.line));
                }
            }
        };

        Ok(token)
    }
}
