use crate::lexers::tokens::{Token, TokenKind};
use crate::grammar::expression::Exp;

struct Parser {
    tokens : Vec<Token>,
    current : usize
}

impl Parser {
    fn new(tokens: Vec<Token>) -> self {
        self {
            tokens,
            0,
        }
    }

    fn get_current(&self) -> &Token {
        &self.tokens[self.current]
    }
 }