#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
     // Single-character tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Semicolon, Colon,
    Minus, Plus, Star, Slash, Percent,

    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords 
    And, Or, Not,
    If, Else,
    While, For,
    Fun, Return,
    Const, Var,
    True, False, Nil,
    Print,
    Class,

    // Special
    Eof,
    Error,   
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type:  TokenKind,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type : TokenKind, lexeme : String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}