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
            current: 0,
        }
    }

    fn get_current(&self) -> &Token {
        &self.tokens[self.current];
    }

    fn get_previous(&self) -> &Token {
        &self.tokens[self.current - 1];
    }

    fn is_at_end(&self) -> bool {
        self.get_current().token_type == TokenKind::Eof;
    }

    fn step(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.get_previous()
    }

 }