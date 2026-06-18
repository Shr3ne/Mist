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

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.step();
        }

        let txt: String = self.source[self.start .. self.current].iter().collect();

        let token = match txt.as_str() {
            "and"    => TokenKind::And,
            "class"  => TokenKind::Class,
            "else"   => TokenKind::Else,
            "false"  => TokenKind::False,
            "for"    => TokenKind::For,
            "fun"    => TokenKind::Fun,
            "if"     => TokenKind::If,
            "null"    => TokenKind::Null,
            "or"     => TokenKind::Or,
            "print"  => TokenKind::Print,
            "not"    => TokenKind::Not,
            "var"    => TokenKind::Var,
            "return" => TokenKind::Return,
            "true"   => TokenKind::True,
            "const"    => TokenKind::Const,
            "while"  => TokenKind::While,

            _ => TokenKind::Identifier(txt),
        };


        self.tokenize(token);
    }

    fn is_alphabet(&self, current_char : char) -> bool {
        current_char.is_ascii_alphabetic() || current_char == '_'
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
            '!' => {let token = if self.peek() == '=' {self.step(); TokenKind::BangEqual} else {TokenKind::Bang};
                    self.tokenize(token);
            },
            '=' => {let token = if self.peek() == '=' {self.step(); TokenKind::EqualEqual} else {TokenKind::Equal};
                    self.tokenize(token);
            },
            '>' => {let token = if self.peek() == '=' {self.step(); TokenKind::GreaterEqual} else {TokenKind::Greater};
                    self.tokenize(token);
            },
            '<' => {let token = if self.peek() == '=' {self.step(); TokenKind::LessEqual} else {TokenKind::Less};
                    self.tokenize(token);
            },

            '0'..='9' => self.number(),
            '\n' => self.line += 1,

            next_char if self.is_alphabet(next_char) => self.identifier(),

            _ => {}

        }
    }
}



