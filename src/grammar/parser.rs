use crate::lexers::tokens::{Token, TokenKind};
use crate::grammar::expression::Exp;

pub struct Parser {
    tokens : Vec<Token>,
    current : usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn get_current(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn get_previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.get_current().token_type == TokenKind::Eof
    }

    fn step(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.get_previous()
    }

    fn check_token(&self, token_type: &TokenKind) -> bool{
        if self.is_at_end() {
            return false;
        }

        &self.get_current().token_type == token_type
    }

    fn match_tokens(&mut self, types: &[TokenKind]) -> bool {
        for t in types {
            if self.check_token(t) {
                self.step();
                return true;
            }
        }

        false
    }

    fn primary(&mut self) -> Result<Exp, String> {
        let token = self.step();

        match &token.token_type {   
            TokenKind::Number(value) => {
                Ok(Exp::Literal { value: TokenKind::Number(*value) })
            },
            
            _ => Err("Error: I was expecting a number here!".to_string())
        }
    }

    fn comparison(&mut self) -> Result<Exp, String>  {
        let mut expr = match self.primary() {
                Ok(ast_node) => ast_node,
                Err(e) => return Err(e),
        };

        // 2. Loop as long as we see comparison operators
        while self.match_tokens(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.get_previous().clone();
            
            let mut right = match self.primary() {
                Ok(ast_node) => ast_node,
                Err(e) => return Err(e),
            };

            expr = Exp::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }
    // Check for == or !=
    fn equality(&mut self) -> Result<Exp, String> {
        let mut expr = self.comparison()?; 

        while self.match_tokens(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
            let operator = self.get_previous().clone();
            
            let right = self.comparison()?; 
            
            expr = Exp::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    pub fn parse_expression(&mut self) -> Result<Exp, String> {
        self.equality()
    }
 }