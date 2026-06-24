use std::collections::HashMap;
use crate::value::MistValue;

#[derive(Debug, Clone)]
pub struct Vars {
    values: HashMap<String, MistValue>,
}

impl Vars {
    pub fn new() -> Self {
        Self {
            values: HashMap::new()
        }
    }

    pub fn set(&mut self, name: String, value: MistValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<MistValue, String> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else {
            Err(format!("Error: Undefined variable '{}'.", name))
        }
    }

    pub fn assign(&mut self, name: String, value: MistValue) -> Result<(), String> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            Err(format!("Error: Undefined variable '{}'.", name))
        }
    }
}