use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod token;

use crate::token::*;

mod scanner {
    use crate::token::*;
    use std::iter;
    use std::str::Chars;
    #[derive(Debug)]
    pub struct ScannerError {
        pub line: usize,
        pub desc: String,
    }

    #[derive(Debug)]
    pub struct Scanner<'a> {
        iter: iter::Peekable<Chars<'a>>,
        buff: Vec<char>,
        line: usize,
    }

    impl<'a> Iterator for Scanner<'a> {
        type Item = Result<Token, ScannerError>;
        //type Item = Token;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(c) = self.iter.next() {
                match c {
                    '{' => {
                        return self.empty_token(TokenType::LeftBrace, c.to_string());
                    }
                    '}' => {
                        return self.empty_token(TokenType::RightBrace, c.to_string());
                    }
                    '(' => {
                        return self.empty_token(TokenType::LeftParen, c.to_string());
                    }
                    ')' => {
                        return self.empty_token(TokenType::RightParen, c.to_string());
                    }
                    '+' => {
                        return self.empty_token(TokenType::Plus, c.to_string());
                    }
                    ',' => {
                        return self.empty_token(TokenType::Comma, c.to_string());
                    }
                    '.' => {
                        return self.empty_token(TokenType::Dot, c.to_string());
                    }
                    '-' => {
                        return self.empty_token(TokenType::Minus, c.to_string());
                    }
                    ';' => {
                        return self.empty_token(TokenType::Semicolon, c.to_string());
                    }
                    '*' => {
                        return self.empty_token(TokenType::Star, c.to_string());
                    }
                    '=' => match self.iter.peek() {
                        Some('=') => {
                            self.iter.next();
                            return self.empty_token(TokenType::EqualEqual, "==".to_string());
                        }
                        _ => {
                            return Some(Ok(Token {
                                kind: TokenType::Equal,
                                lexeme: c.to_string(),
                                line: self.line,
                                value: None,
                            }))
                        }
                    },
                    '!' => match self.iter.peek() {
                        Some('=') => {
                            self.iter.next();
                            return self.empty_token(TokenType::BangEqual, "!=".to_string());
                        }
                        _ => {
                            return self.empty_token(TokenType::Bang, c.to_string());
                        }
                    },
                    '<' => match self.iter.peek() {
                        Some('=') => {
                            self.iter.next();
                            return self.empty_token(TokenType::LessEqual, "<=".to_string());
                        }
                        _ => return self.empty_token(TokenType::Less, c.to_string()),
                    },
                    '>' => match self.iter.peek() {
                        Some('=') => {
                            self.iter.next();
                            return self.empty_token(TokenType::GreaterEqual, ">=".to_string());
                        }
                        _ => return self.empty_token(TokenType::Greater, c.to_string()),
                    },
                    '/' => match self.iter.peek() {
                        Some('/') => {
                            while let Some(nchar) = self.iter.peek() {
                                if *nchar == '\n' {
                                    break;
                                }
                                self.iter.next();
                            }
                        }
                        _ => {
                            return self.empty_token(TokenType::Slash, c.to_string());
                        }
                    },
                    '"' => loop {
                        let cnext = self.iter.peek();
                        match cnext {
                            Some('"') => {
                                let ret = self
                                    .token(TokenType::String, self.buff.iter().collect::<String>());
                                self.iter.next();
                                self.buff.clear();
                                return ret;
                            }
                            Some(c) => {
                                self.buff.push(*c);
                                self.iter.next();
                            }
                            None => return self.scan_err("Unterminated string literal"),
                        }
                    },
                    '\t' => continue,
                    '\r' => continue,
                    ' ' => continue,
                    '\n' => self.line += 1,
                    _ => return self.scan_err("invalid character"),
                };
            }
            None
        }
    }

    impl<'a> Scanner<'a> {
        pub fn new(iter: Chars<'a>) -> Self {
            Scanner {
                iter: iter.peekable(),
                buff: vec![],
                line: 1,
            }
        }

        fn empty_token(
            &self,
            kind: TokenType,
            lexeme: String,
        ) -> Option<Result<Token, ScannerError>> {
            Some(Ok(Token {
                kind,
                lexeme,
                line: self.line,
                value: None,
            }))
        }

        fn token(&self, kind: TokenType, lexeme: String) -> Option<Result<Token, ScannerError>> {
            Some(Ok(Token {
                kind,
                lexeme,
                line: self.line,
                value: None,
            }))
        }

        fn scan_err(&self, message: &str) -> Option<Result<Token, ScannerError>> {
            Some(Err(ScannerError {
                line: self.line,
                desc: message.to_string(),
            }))
        }
    }
}

use crate::scanner::*;

fn run(source: &str) {
    //println!("{} lines of source", source.lines().count());
    let tokens: Vec<Result<Token, ScannerError>> = Scanner::new(source.chars()).collect();
    println!("{} tokens from source", tokens.len());
    for tok in tokens.iter() {
        if let Ok(tok) = tok {
            println!("   {}", tok);
        } else {
            println!("   {:?}", tok);
        }
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("Really shouldn't let this stay");
    run(&source);
}

fn run_prompt() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let chars_read = handle.read_line(&mut buffer).unwrap();
        if chars_read == 0 {
            break;
        }
        let line = buffer.trim();
        run(line);
        println!("->{}", line);
        //io::stdout().flush().unwrap();
        buffer.clear();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, Welcome to Lox in Rust!");
    match args.len() {
        1 => {
            println!("Entering interactive mode(CTRL+D to exit):");
            run_prompt();
            println!("...Exiting");
        }
        2 => {
            println!("processing file: {}", args[1]);
            run_file(&args[1])
        }
        _ => println!("USAGE: {} [script-file]", args[0]),
    }
}
