pub mod lexers;
pub mod parser;
pub mod ast;
pub mod value;
pub mod interpreter;

use lexers::scanner::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use value::MistValue;

fn main() {
    // A classic order-of-operations test!
    let source_code = "print 10 + 10;"; 
    println!("Executing Mist Code: '{}'\n", source_code);

    let mut lexer = Lexer::new(source_code);
    lexer.scan_source(); 

    let mut parser = Parser::new(lexer.tokens);

    match parser.parse() {
        Ok(statements) => {
           match Interpreter::interpret(&statements) {
                Ok(results) => {
                    for result in results {
                        match result {
                            MistValue::Null => {}, 
                            other => println!("{}", other),
                        }
                    }
                },

                Err(e) => println!("Runtime Error: {}", e),
            }
        },
        
        Err(e) => println!("Parse Error: {}", e),
    }
}