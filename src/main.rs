mod token;
mod token_type;
mod scanner;
mod expression;
mod parser;



use std::env;
use std::fs;
use std::process;
use std::io::{self, Write, BufRead};

use scanner::Scanner;
use parser::Parser;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("couldn't read the file");
    let had_error = run(source);
    if had_error {
        process::exit(65);
    }
}

fn run(source: String) -> bool {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().clone();

    let errors = scanner.errors();
    for err in errors {
        eprintln!("[line {}] Error: {}", err.line, err.message);
    }
    if !errors.is_empty() {
        return true;
    }

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(expr) => {
            println!("{}", expression::print(&expr));
            false
        }
        // parser already printed the error
        Err(_) => true,
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = stdin.lock().read_line(&mut line).unwrap();

        if bytes_read == 0 {
            break;
        }

        run(line);
    }
}