pub mod lexers;
pub mod grammar;
pub mod value;
pub mod interpreter;

use lexers::scanner::Lexer;
use grammar::parser::Parser;
use interpreter::Interpreter;

fn main() {
    // A classic order-of-operations test!
    let source_code = "5 + 10 == 25"; 
    println!("Executing Mist Code: '{}'\n", source_code);

    let mut lexer = Lexer::new(source_code);
    lexer.scan_source(); 

    let mut parser = Parser::new(lexer.tokens);

    match parser.parse_expression() {
        Ok(ast) => {
            match Interpreter::evaluate(&ast) {
                Ok(result) => println!("Final Answer: {}", result),
                Err(e) => println!("Runtime Error: {}", e),
            }
        },
        Err(e) => println!("Parse Error: {}", e),
    }
}