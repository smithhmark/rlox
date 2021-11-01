use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, Welcome to Lox in Rust!");
    if args.len() == 1 {
        println!("Entering interactive mode:");
        println!("...Exiting");

    } else {
        println!("Here will be non-interactive mode, eventually");
    }
}
