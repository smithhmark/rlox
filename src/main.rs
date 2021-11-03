use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

mod scanner;
mod token;

use crate::scanner::*;
use crate::token::*;

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
