mod token;
mod token_type;
mod scanner;
mod expression;
mod parser;
mod value;
mod interpreter;
mod stmt;
mod environment;



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
    let mut interpreter = Interpreter::new();
    match run(source, &mut interpreter) {

        Err(LoxError::Syntax) => process::exit(65),
        Err(LoxError::Runtime) => process::exit(70),
        Ok(()) => {}
    }
}

enum LoxError {
    Syntax,
    Runtime,
}

fn run(source: String, interpreter: &mut Interpreter<io::Stdout>) -> Result<(), LoxError> {

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
    let statements = parser.parse().map_err(|_| LoxError::Syntax)?;

    interpreter.interpret(&statements).map_err(|err| {
        eprintln!("{}\n[line {}]", err.message, err.token.line);
        LoxError::Runtime
    })
}

fn run_prompt() {
  //one interpreter for the entire session
    let mut interpreter = Interpreter::new();
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        let bytes_read = stdin.lock().read_line(&mut line).unwrap();

        if bytes_read == 0 {
            break;
        }

        //here our errors are already reported, so we can make the repl keep going...
        let _ = run(line, &mut interpreter);
    }
}