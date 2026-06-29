use crate::lexers::tokens::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Exp {
    Binary { 
        left: Box<Exp>,
        operator: Token,
        right: Box<Exp>
    },
    
    Literal {
        value: TokenKind,
    },

    Unary {
        operator: Token,
        right: Box<Exp>
    },

    Variable {
        name: Token
    },

    Assign {
        name: Token,
        value: Box<Exp>,
    },
}

#[derive(Debug, Clone)]
pub enum Smt {

    Print(Exp),
    Expression(Exp),
    Block(Vec<Smt>),
    Var {
        name: Token,
        init: Exp,
    },

    If {
        condition: Exp,
        then_branch: Box<Smt>,
        else_branch: Option<Box<Smt>>,
    },
}