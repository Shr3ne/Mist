use crate::ast::ast::{Exp, Smt};
use crate::lexers::tokens::TokenKind;
use crate::value::MistValue;
use crate::variable::Vars;

pub struct Interpreter {
    pub env: Vars,
}

impl Interpreter {
     pub fn new() -> Self {
        Self {
            env: Vars::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Smt]) -> Result<Vec<MistValue>, String> {
        let mut results = Vec::new();
        for smt in statements {
            results.push(self.execute(smt)?);
        }

        Ok(results)
    }

    pub fn execute(&mut self, smt: &Smt) -> Result<MistValue, String> {
        match smt {
            Smt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value);

                Ok(MistValue::Null)
            },
            Smt::Expression(expr) => {
                let value = self.evaluate(expr)?;

                Ok(value)
            },

            Smt::If { condition, then_branch, else_branch } => {
                let cond_value = self.evaluate(condition)?;
            
                let truth = !matches!(cond_value, MistValue::Null | MistValue::Boolean(false));

                if truth {
                    self.execute(then_branch)?;
                } else if let Some(else_stmt) = else_branch {
                    self.execute(else_stmt)?;
                }

                Ok(MistValue::Null)
            },

            Smt::Loop { condition, body } => {
                loop {
                    let cond_val = self.evaluate(condition)?;
                    
                    // If the condition evaluates to false or null, break the loop
                    if matches!(cond_val, MistValue::Null | MistValue::Boolean(false)) {
                        break;
                    }
                    
                    self.execute(body)?;
                }
                
                Ok(MistValue::Null)
            }

            Smt::Var { name, init } => {
                let value = self.evaluate(init)?;

                self.env.set(name.lexeme.clone(), value);

                Ok(MistValue::Null)
            },

            Smt::Block(statements)=> {
                let previous_env = self.env.clone();
    
                self.env = Vars::new_local(previous_env);

                for statement in statements {
                 self.execute(statement)?;
                }

                if let Some(parent) = self.env.enclosure.take() {
                     self.env = *parent;
                }
    
                Ok(MistValue::Null)
            }
        }
    }

    pub fn evaluate(&mut self, expr: &Exp) -> Result<MistValue, String> {
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
                let right_value = self.evaluate(right)?;

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
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                match operator.token_type {
                    // --- ARITHMETIC ---
                    TokenKind::Plus => {
                        match (&left_val, &right_val) {

                                (MistValue::Number(l), MistValue::Number(r)) => {
                                    Ok(MistValue::Number(l + r))
                                },
    
                                (MistValue::String(l), MistValue::String(r)) => {
                                    Ok(MistValue::String(format!("{}{}", l, r)))
                                },
                                (MistValue::String(l), MistValue::Number(r)) => {
                                    Ok(MistValue::String(format!("{}{}", l, r)))
                                },
                                (MistValue::Number(l), MistValue::String(r)) => {
                                    Ok(MistValue::String(format!("{}{}", l, r)))
                                },
                                _ => Err(format!("[Line {}] Runtime Error: Operands must be two numbers or strings.", operator.line))
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
            },

            Exp::Variable { name } => {
                self.env.get(&name.lexeme)
            },

            Exp::Assign { name, value } => {
                let eval_value = self.evaluate(value)?;

                self.env.assign(name.lexeme.clone(), eval_value.clone())?;
                Ok(eval_value) 
            }
        }
    }
}