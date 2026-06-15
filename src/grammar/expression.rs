use crate::lexers::tokens::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Exp {
    Binary { 
        left: Box<Exp>,
        operator: Token,
        right: Box<Exp>
    },

    Grouping {
        expression: Box<Exp> 
    },

    Literal {
        value: TokenKind,
    },

    Unary {
        opterator: Token,
        right: Box<Exp>
    }
}