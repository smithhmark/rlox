use std::env;
use std::io::{self, BufRead, Write};

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
        println!("->{}", line);
        io::stdout().flush().unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, Welcome to Lox in Rust!");
    if args.len() == 1 {
        println!("Entering interactive mode:");
        run_prompt();
        println!("...Exiting");

    } else {
        println!("Here will be non-interactive mode, eventually");
    }
}
