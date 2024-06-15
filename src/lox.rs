use crate::scanner;
use crate::token::Token;
use std::io::prelude::*;
use std::{fs, io, path, process};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub fn run(&mut self, source: String) {
        let mut scanner: scanner::Scanner = scanner::Scanner::new(&source);
        let mut tokens: Vec<Token> = Vec::new();
        match scanner.scan_tokens() {
            Ok(ts) => tokens = ts,
            Err(errors) => errors
                .iter()
                .for_each(|err| self.report(err.line(), format!("{}", err))),
        };

        println!("Current Tokens: {:?}", tokens);
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let mut input = String::new();
        let stdin = io::stdin();
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            stdin.lock().read_line(&mut input)?;
            self.run(input.clone());
            input.clear();
            self.had_error = false;
        }
    }

    pub fn runfile(&mut self, path: path::PathBuf) -> io::Result<()> {
        let mut source = String::new();
        let mut file = fs::File::open(path)?;

        file.read_to_string(&mut source)?;
        self.run(source);

        if self.had_error {
            process::exit(1);
        }

        Ok(())
    }

    pub fn report(&mut self, line: usize, message: String) {
        println!("[line {}] Error: {}", line, message);
        self.had_error = true;
    }
}
