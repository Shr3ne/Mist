use crate::lexers::tokens::{Token, TokenKind};
use crate::ast::ast::{Exp, Smt};

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

    fn error(&self, token: &Token, message: &str) -> String {
        if token.token_type == TokenKind::Eof {
            format!("[Line {}] Error at end: {}", token.line, message)
        } else {
            format!("[Line {}] Error at '{}': {}", token.line, token.lexeme, message)
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

    // STATEMENT LIST
    pub fn parse(&mut self) -> Result<Vec<Smt>, String> {
        let mut smt_list = Vec::new();
        while !self.is_at_end() {
            smt_list.push(self.declaration()?);
        }
        Ok(smt_list)
    }

    fn declaration(&mut self) -> Result<Smt, String> {
        if self.match_tokens(&[TokenKind::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Smt, String> {
        let name = self.step().clone();
        
        match name.token_type {
            TokenKind::Identifier(_) => {},
            _ => return Err(self.error(&name, "Expect variable name.")),
        }

        let init = if self.match_tokens(&[TokenKind::Equal]) {
            self.parse_expression()?
        } else {
            Exp::Literal { value: TokenKind::Null }
        };

        if self.match_tokens(&[TokenKind::Semicolon]) {
            Ok(Smt::Var { name, init })
        } else {
            Err("Parse Error: Expect ';' after variable declaration.".to_string())
        }
    }

    pub fn statement(&mut self) -> Result<Smt, String> {
        if self.match_tokens(&[TokenKind::If]) {
            self.if_smt()
        } else if self.match_tokens(&[TokenKind::Print]) {
            self.print_smt()
        } else if self.match_tokens(&[TokenKind::LeftBrace]) {
            Ok(Smt::Block(self.scope()?))
        } else {
            self.expression_smt()
        }
    }

    pub fn scope(&mut self) -> Result<Vec<Smt>, String> {
       let mut statements = Vec::new();

        while !self.check_token(&TokenKind::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        if !self.match_tokens(&[TokenKind::RightBrace]) {
            return Err("Parse Error: Expect '}' after block.".to_string());
        }

        Ok(statements)
    }

    pub fn print_smt(&mut self) -> Result<Smt, String> {
       let value = self.parse_expression()?;

        if self.match_tokens(&[TokenKind::Semicolon]) {
            Ok(Smt::Print(value))
        } else {
            Err("Parse Error: Expected ';' after ".to_string())
        }
    }

    pub fn if_smt(&mut self) -> Result<Smt, String> {
        if !self.match_tokens(&[TokenKind::LeftParen]) {
            return Err("Parse Error: Expect '(' after 'if'.".to_string());
        }
        
        let condition = self.parse_expression()?;
        
        if !self.match_tokens(&[TokenKind::RightParen]) {
            return Err("Parse Error: Expect ')' after if condition.".to_string());
        }

        let then_branch = Box::new(self.statement()?);
        
        let else_branch = if self.match_tokens(&[TokenKind::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Smt::If { condition, then_branch, else_branch })
    }

    pub fn expression_smt(&mut self) -> Result<Smt, String> {
        let expr = self.parse_expression()?;
        
        if self.match_tokens(&[TokenKind::Semicolon]) {
            Ok(Smt::Expression(expr))
        } else {
            Err("Parse Error: Expected ';' after ".to_string())
        }
    }

    // EXPRESSION LIST
    fn assignment(&mut self) -> Result<Exp, String> {
        let expr = self.equality()?;

        if self.match_tokens(&[TokenKind::Equal]) {
            let equals = self.get_previous().clone();
            let value = self.assignment()?;

            if let Exp::Variable { name } = expr {
                return Ok(Exp::Assign { name, value: Box::new(value) });
            }

            return Err(self.error(&equals, "Invalid assignment target."));
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Exp, String> {
        let token = self.step().clone();

        match &token.token_type {   
            TokenKind::Number(value) => {
                Ok(Exp::Literal { value: TokenKind::Number(*value) })
            },

            TokenKind::True => {
                Ok(Exp::Literal { value: TokenKind::True })
            },
            TokenKind::False => {
                Ok(Exp::Literal { value: TokenKind::False })
            },
            TokenKind::Null => {
                Ok(Exp::Literal { value: TokenKind::Null })
            },

            TokenKind::String(s) => {
                Ok(Exp::Literal { value: TokenKind::String(s.clone()) })
            },

            TokenKind::Identifier(_) => {
                Ok(Exp::Variable { name: token.clone() })
            },

            TokenKind::Error(message) => {
                Err(self.error(&token, message))
            },
            
            _ => Err(self.error(&token, "Expect expression!!"))
        }
    }

    fn comparison(&mut self) -> Result<Exp, String>  {
        let mut expr = self.term()?;

        while self.match_tokens(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.get_previous().clone();
            
            let right = match self.term() {
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

    fn term(&mut self) -> Result<Exp, String> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.get_previous().clone();

            let right = self.factor()?;

            expr = Exp::Binary {
                 left: Box::new(expr), 
                 operator, 
                 right: Box::new(right) };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Exp, String> {
        let mut expr= self.unary()?;

        while self.match_tokens(&[TokenKind::Star, TokenKind::Slash]) {
            let operator = self.get_previous().clone();

            let right = self.unary()?;

            expr = Exp::Binary {
                 left: Box::new(expr), 
                 operator, 
                 right: Box::new(right) };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Exp, String> {
        if self.match_tokens(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.get_previous().clone();

            let right = self.unary()?;

            return Ok(Exp::Unary { 
                operator, 
                right: Box::new(right) 
            });
        }

        self.primary()
    }

    pub fn parse_expression(&mut self) -> Result<Exp, String> {
        self.assignment()
    }
 }