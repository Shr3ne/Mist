pub mod lexers;
pub mod parser;
pub mod ast;
pub mod value;
pub mod interpreter;
pub mod variable;

use std::env;
use std::fs;
use std::io::{Write, self};

use lexers::scanner::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn run(code : &str, interpreter : &mut Interpreter) {
    let mut lexer = Lexer::new(code);
    lexer.scan_source();

    let mut parser = Parser::new(lexer.tokens);

     match parser.parse() {
        Ok(statements) => {
            if let Err(e) = interpreter.interpret(&statements) {
                println!("Runtime Error: {}", e);
            }
        },

        Err(e) => println!("Parse Error: {}", e),
    }
}

fn run_terminal() {
    let mut interpreter = Interpreter::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("Welcome to Mist! Type '.end' to quit.");
    
    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let line = line.trim();

        if line == ".end" {
            break;
        }
        if line.is_empty() {
            continue;
        }

        run(line, &mut interpreter);
    }
}

fn run_file(path : &str) {
    let code = fs::read_to_string(path).expect("Failed to read file...");

    let mut interpreter = Interpreter::new();
    run(&code, &mut interpreter);
}

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() > 2 {
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_terminal();
    }
}