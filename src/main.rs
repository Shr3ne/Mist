pub mod lexers;
pub mod parser;
pub mod ast;
pub mod value;
pub mod interpreter;

use lexers::scanner::Lexer;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    // A classic order-of-operations test!
    let source_code = "print 10 + 10; print 5 * 2; 10 + 10 == 20;"; 
    println!("Executing Mist Code: '{}'\n", source_code);

    let mut lexer = Lexer::new(source_code);
    lexer.scan_source(); 

    let mut parser = Parser::new(lexer.tokens);

    match parser.parse() {
        Ok(statements) => {
           match Interpreter::interpret(&statements) {
                Ok(results) => println!("{:?}", results),
                Err(e) => println!("Runtime Error: {}", e),
            }
        },
        
        Err(e) => println!("Parse Error: {}", e),
    }
}