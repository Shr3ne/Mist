use crate::lexers::tokens::{Token, TokenKind};
pub struct Lexer {
    source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Lexer {
    pub fn new(src : &str) -> Lexer {
        Lexer {
            source: src.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_source(&mut self) {
        while (!self.is_at_end()) {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenKind::Eof, "".to_string(), self.line));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn step(&mut self) -> char {
        let next_char = self.source[self.current];
        self.current += 1;

        next_char
    }

    fn tokenize(&mut self, token_type: TokenKind) {
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(token_type, text, self.line));
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.step();
        }

        // check for decimals
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.step();
            
            while self.peek().is_ascii_digit() {
                self.step();
            }
        }

        // extract string
        let value_str: String = self.source[self.start..self.current].iter().collect();
        
        // Parse it into a Rust f64 float!
        let value: f64 = value_str.parse().expect("Failed to parse number.");
        
        // TokenKind::Number(f64)
        self.tokenize(TokenKind::Number(value));
    }
    

    fn scan_token(&mut self) {
        let next_char = self.step();

        match next_char {
            '(' => self.tokenize(TokenKind::LeftParen),
            ')' => self.tokenize(TokenKind::RightParen),
            '{' => self.tokenize(TokenKind::RightBrace),
            '}' => self.tokenize(TokenKind::LeftBrace),
            ',' => self.tokenize(TokenKind::Comma),
            '.' => self.tokenize(TokenKind::Dot),
            ':' => self.tokenize(TokenKind::Colon),
            ';' => self.tokenize(TokenKind::Semicolon),
            '-' => self.tokenize(TokenKind::Minus),
            '+' => self.tokenize(TokenKind::Plus),
            '*' => self.tokenize(TokenKind::Star),
            '/' => self.tokenize(TokenKind::Slash),
            '%' => self.tokenize(TokenKind::Percent),
            '0'..='9' => self.number(),
            '\n' => self.line += 1,

            _ => {}

        }
    }
}



