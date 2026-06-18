mod lexers;
mod grammar;

use lexers::scanner::Lexer;
use lexers::tokens;

use grammar::parser::Parser;

fn main() {
    let source_code = "-10 * 20 < 50 / 15;";

    let mut lexer = Lexer::new(source_code);
    lexer.scan_source();

    let mut parser = Parser::new(lexer.tokens);

    match parser.parse_expression() {
        Ok(ast) => println!("AST:\n{:#?}", ast),
        Err(e) => println!("Parser failed: {}", e),
    }
}
