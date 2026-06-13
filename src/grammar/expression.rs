use crate::lexers::tokens::{Token, TokenKind};

pub enum Exp {
    Binary { 
        L: Box<Exp>,
        Op: Token,
        R: Box<Exp>
    },

    Grouping {
        expression: Box<Exp> 
    },

    Literal {
        Value: TokenKind,
    },

    Unary {
        Op: Token,
        R: Box<Exp>
    }
}