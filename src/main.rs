use std::env;
use std::fmt;
use std::fs;
use std::io::{self, BufRead, Write};
use std::iter;
use std::str::Chars;

#[derive(Debug)]
enum TokenType {
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

    EOF,
}

#[derive(Debug)]
struct Token {
    kind: TokenType,
    lexeme: String,
    line: usize,
    value: Option<usize>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            None => write!(f, "{:?} {} {}", self.kind, self.lexeme, "None"),
            Some(_) => write!(f, "{:?} {} {}", self.kind, self.lexeme, "not yet"),
        }
    }
}

#[derive(Debug)]
struct Scanner<'a>
{
    iter: iter::Peekable<Chars<'a>>,
    buff: Vec<char>,
    line: usize,
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.iter.next() {
            match c {
                '{' => {
                    return Some(Token {
                        kind: TokenType::LeftBrace,
                        lexeme: c.to_string(),
                        line: self.line,
                        value: None,
                    })
                }
                '}' => {
                    return Some(Token {
                        kind: TokenType::RightBrace,
                        lexeme: c.to_string(),
                        line: self.line,
                        value: None,
                    })
                }
                '(' => {
                    return Some(Token {
                        kind: TokenType::LeftParen,
                        lexeme: c.to_string(),
                        line: self.line,
                        value: None,
                    })
                }
                ')' => {
                    return Some(Token {
                        kind: TokenType::RightParen,
                        lexeme: c.to_string(),
                        line: self.line,
                        value: None,
                    })
                }
                '+' => {
                    return Some(Token {
                        kind: TokenType::Plus,
                        lexeme: c.to_string(),
                        line: self.line,
                        value: None,
                    })
                }
                ',' => {
                    return Some(Token {
                        kind: TokenType::Comma,
                        lexeme: c.to_string(),
                        line: self.line,
                        value: None,
                    })
                }
                '.' => {
                    return Some(Token {
                        kind: TokenType::Dot,
                        lexeme: c.to_string(),
                        line: self.line,
                        value: None,
                    })
                }
                '-' => {
                    return Some(Token {
                        kind: TokenType::Minus,
                        lexeme: c.to_string(),
                        line: self.line,
                        value: None,
                    })
                }
                ';' => {
                    return Some(Token {
                        kind: TokenType::Semicolon,
                        lexeme: c.to_string(),
                        line: self.line,
                        value: None,
                    })
                }
                '*' => {
                    return Some(Token {
                        kind: TokenType::Star,
                        lexeme: c.to_string(),
                        line: self.line,
                        value: None,
                    })
                }
                '=' => match self.iter.peek() {
                    Some('=') => {
                        self.iter.next();
                        return Some(Token {
                            kind: TokenType::EqualEqual,
                            lexeme: "==".to_string(),
                            line: self.line,
                            value: None,
                        });
                    }
                    _ => {
                        return Some(Token {
                            kind: TokenType::Equal,
                            lexeme: c.to_string(),
                            line: self.line,
                            value: None,
                        })
                    }
                },
                '!' => match self.iter.peek() {
                    Some('=') => {
                        self.iter.next();
                        return Some(Token {
                            kind: TokenType::BangEqual,
                            lexeme: "!=".to_string(),
                            line: self.line,
                            value: None,
                        });
                    }
                    _ => {
                        return Some(Token {
                            kind: TokenType::Bang,
                            lexeme: c.to_string(),
                            line: self.line,
                            value: None,
                        })
                    }
                },
                '<' => match self.iter.peek() {
                    Some('=') => {
                        self.iter.next();
                        return Some(Token {
                            kind: TokenType::LessEqual,
                            lexeme: "<=".to_string(),
                            line: self.line,
                            value: None,
                        });
                    }
                    _ => {
                        return Some(Token {
                            kind: TokenType::Less,
                            lexeme: c.to_string(),
                            line: self.line,
                            value: None,
                        })
                    }
                },
                '>' => match self.iter.peek() {
                    Some('=') => {
                        self.iter.next();
                        return Some(Token {
                            kind: TokenType::GreaterEqual,
                            lexeme: ">=".to_string(),
                            line: self.line,
                            value: None,
                        });
                    }
                    _ => {
                        return Some(Token {
                            kind: TokenType::Greater,
                            lexeme: c.to_string(),
                            line: self.line,
                            value: None,
                        })
                    }
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
                        return Some(Token {
                            kind: TokenType::Slash,
                            lexeme: c.to_string(),
                            line: self.line,
                            value: None,
                        })
                    }


                }
                '\n' => self.line += 1,
                _ => continue,
            };
        }
        None
    }
}

impl<'a> Scanner<'a> {
    fn new(iter: Chars<'a>) -> Self {
        Scanner {
            iter: iter.peekable(),
            buff: vec![],
            line: 1,
        }
    }
}

fn run(source: &str) {
    //println!("{} lines of source", source.lines().count());
    let tokens: Vec<Token> = Scanner::new(source.chars()).collect();
    println!("{} tokens from source", tokens.len());
    for tok in tokens.iter() {
        println!("   {}", tok);
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
