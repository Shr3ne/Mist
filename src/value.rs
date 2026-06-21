use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum MistValue {
    Number(f64),
    Boolean(bool),
    String(String),
    Null
}

impl fmt::Display for MistValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MistValue::Number(n) => write!(f, "{}", n),
            MistValue::Boolean(b) => write!(f, "{}", b),
            MistValue::String(s) => write!(f, "{}", s),
            MistValue::Null => write!(f, "Null"),
        }
    }
}