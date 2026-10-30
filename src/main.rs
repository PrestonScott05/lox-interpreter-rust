mod token;
mod token_type;
mod scanner;
mod expression;
mod parser;
mod value;
mod interpreter;



use std::env;
use std::fs;
use std::process;
use std::io::{self, Write, BufRead};

use scanner::Scanner;
use parser::Parser;
use interpreter::Interpreter;



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
    let interpreter = Interpreter::new();
    match run(source, &interpreter) {

        Err(LoxError::Syntax) => process::exit(65),
        Err(LoxError::Runtime) => process::exit(70),
        Ok(()) => {}
    }
}

enum LoxError {
    Syntax,
    Runtime,
}

fn run(source: String, interpreter: &Interpreter) -> Result<(), LoxError> {

    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens().clone();

    let errors = scanner.errors();
    for err in errors {
        eprintln!("[line {}] Error: {}", err.line, err.message);
    }
    if !errors.is_empty() {
        return Err(LoxError::Syntax);
    }

    let mut parser = Parser::new(tokens);
    //means the parser already printed the error lol
    let expr = parser.parse().map_err(|_| LoxError::Syntax)?;

    interpreter.interpret(&expr).map_err(|err| {
        eprintln!("{}\n[line {}]", err.message, err.token.line);
        LoxError::Runtime
    })
}

fn run_prompt() {
  //one interpreter for the entire session
    let interpreter = Interpreter::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = stdin.lock().read_line(&mut line).unwrap();

        if bytes_read == 0 {
            break;
        }

        // errors already reported; REPL keeps going
        let _ = run(line, &interpreter);
    }
}