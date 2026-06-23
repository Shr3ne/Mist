use crate::ast::ast::{Exp, Smt};
use crate::lexers::tokens::TokenKind;
use crate::value::MistValue;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(statements: &[Smt]) -> Result<Vec<MistValue>, String> {
        let mut results = Vec::new();
        for smt in statements {
            results.push(Self::execute(smt)?);
        }

        Ok(results)
    }

    pub fn execute(smt: &Smt) -> Result<MistValue, String> {
        match smt {
            Smt::Print(expr) => {
                let value = Self::evaluate(expr)?;
                println!("{}", value);
                Ok(MistValue::Null)
            },
            Smt::Expression(expr) => {
                let value = Self::evaluate(expr)?;
                Ok(value)
            }
        }
    }

    pub fn evaluate(expr: &Exp) -> Result<MistValue, String> {
        match expr {
            Exp::Literal { value } => match value {
                TokenKind::Number(n) => Ok(MistValue::Number(*n)),
                TokenKind::True => Ok(MistValue::Boolean(true)),
                TokenKind::False => Ok(MistValue::Boolean(false)),
                TokenKind::Null => Ok(MistValue::Null),
                TokenKind::String(s) => Ok(MistValue::String(s.clone())), 
                _ => Err("Runtime Error: Invalid literal value.".to_string()),
            },

            Exp::Unary { operator, right } => {
                let right_value = Self::evaluate(right)?;

                match operator.token_type {
                    TokenKind::Minus => {
                        if let MistValue::Number(n) = right_value {
                            Ok(MistValue::Number(-n))
                        } else {
                            Err(format!("[Line {}] Runtime Error: Operand must be a number.", operator.line))
                        }
                    }
                    TokenKind::Bang => {
                        let is_truthy = !matches!(right_value, MistValue::Null | MistValue::Boolean(false));
                        Ok(MistValue::Boolean(!is_truthy))
                    }
                    _ => Err("Runtime Error: Unknown unary operator.".to_string()),
                }
            },

            Exp::Binary { left, operator, right } => {
                let left_val = Self::evaluate(left)?;
                let right_val = Self::evaluate(right)?;

                match operator.token_type {
                    // --- ARITHMETIC ---
                    TokenKind::Plus => {
                        if let (MistValue::Number(l), MistValue::Number(r)) = (&left_val, &right_val) {
                            Ok(MistValue::Number(l + r))
                        } else {
                            Err(format!("[Line {}] Runtime Error: Operands must be numbers.", operator.line))
                        }
                    }
                    TokenKind::Minus => {
                        if let (MistValue::Number(l), MistValue::Number(r)) = (&left_val, &right_val) {
                            Ok(MistValue::Number(l - r))
                        } else {
                            Err(format!("[Line {}] Runtime Error: Operands must be numbers.", operator.line))
                        }
                    }
                    TokenKind::Star => {
                        if let (MistValue::Number(l), MistValue::Number(r)) = (&left_val, &right_val) {
                            Ok(MistValue::Number(l * r))
                        } else {
                            Err(format!("[Line {}] Runtime Error: Operands must be numbers.", operator.line))
                        }
                    }
                    TokenKind::Slash => {
                        if let (MistValue::Number(l), MistValue::Number(r)) = (&left_val, &right_val) {
                            if *r == 0.0 {
                                return Err(format!("[Line {}] Runtime Error: Division by zero.", operator.line));
                            }
                            Ok(MistValue::Number(l / r))
                        } else {
                            Err(format!("[Line {}] Runtime Error: Operands must be numbers.", operator.line))
                        }
                    }

                    // --- COMPARISONS ---
                    TokenKind::Greater => {
                        if let (MistValue::Number(l), MistValue::Number(r)) = (&left_val, &right_val) {
                            Ok(MistValue::Boolean(l > r))
                        } else {
                            Err(format!("[Line {}] Runtime Error: Operands must be numbers.", operator.line))
                        }
                    }
                    TokenKind::GreaterEqual => {
                        if let (MistValue::Number(l), MistValue::Number(r)) = (&left_val, &right_val) {
                            Ok(MistValue::Boolean(l >= r))
                        } else {
                            Err(format!("[Line {}] Runtime Error: Operands must be numbers.", operator.line))
                        }
                    }
                    TokenKind::Less => {
                        if let (MistValue::Number(l), MistValue::Number(r)) = (&left_val, &right_val) {
                            Ok(MistValue::Boolean(l < r))
                        } else {
                            Err(format!("[Line {}] Runtime Error: Operands must be numbers.", operator.line))
                        }
                    }
                    TokenKind::LessEqual => {
                        if let (MistValue::Number(l), MistValue::Number(r)) = (&left_val, &right_val) {
                            Ok(MistValue::Boolean(l <= r))
                        } else {
                            Err(format!("[Line {}] Runtime Error: Operands must be numbers.", operator.line))
                        }
                    }

                    // --- EQUALITY ---
                    TokenKind::EqualEqual => Ok(MistValue::Boolean(left_val == right_val)),
                    TokenKind::BangEqual => Ok(MistValue::Boolean(left_val != right_val)),

                    _ => Err(format!("[Line {}] Runtime Error: Unknown binary operator.", operator.line)),
                }
            }
        }
    }
}