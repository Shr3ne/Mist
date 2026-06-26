pub mod lexers;
pub mod parser;
pub mod ast;
pub mod value;
pub mod interpreter;
pub mod variable;

use lexers::scanner::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let source_code = "
        var a = 10;
        var b = 20;
        if (a > b) print a; else print b;
    "; 
    println!("Executing Mist Code: '{}'\n", source_code);

    let mut lexer = Lexer::new(source_code);
    lexer.scan_source(); 

    let mut parser = Parser::new(lexer.tokens);
    let mut interpreter = Interpreter::new();

    match parser.parse() {
        Ok(statements) => {
            if let Err(e) = interpreter.interpret(&statements) {
                println!("Runtime Error: {}", e);
            }
        },
        Err(e) => println!("Parse Error: {}", e),
    }
}