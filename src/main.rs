pub mod lexers;
pub mod parser;
pub mod ast;
pub mod value;
pub mod interpreter;
pub mod variable;

use lexers::scanner::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use value::MistValue;

fn main() {
    // A classic order-of-operations test!
    let source_code = "print 10 + 10;print 10 + 10 == 20;"; 
    println!("Executing Mist Code: '{}'\n", source_code);

    let mut lexer = Lexer::new(source_code);
    lexer.scan_source(); 

    let mut parser = Parser::new(lexer.tokens);

    match parser.parse() {
        Ok(statements) => {
            if let Err(e) = Interpreter::interpret(&statements) {
                println!("Runtime Error: {}", e);
            }
        },
        Err(e) => println!("Parse Error: {}", e),
    }
}