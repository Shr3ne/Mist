mod lexers;

use lexers::scanner::Lexer;
use lexers::tokens;

fn main() {
    let source_code = "123.67 + 45 and 29 < 23 <= 21;";
    println!("Scanning source:");

    let mut lexer = Lexer::new(source_code);
    lexer.scan_source();

    println!("Tokens Generated");
    
    for token in &lexer.tokens {
        println!("{:?}", token);
    }
}
