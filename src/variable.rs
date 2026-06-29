use std::collections::HashMap;
use crate::value::MistValue;

#[derive(Debug, Clone)]
pub struct Vars {
    values: HashMap<String, MistValue>,
    pub enclosure: Option<Box<Vars>>,
}

impl Vars {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosure: None,
        }
    }

    pub fn new_local(enclosure: Vars) -> Self {
        Self {
            values: HashMap::new(),
            enclosure: Some(Box::new(enclosure)),
        }
    }

    pub fn set(&mut self, name: String, value: MistValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<MistValue, String> {
        if let Some(value) = self.values.get(name) {
            Ok(value.clone())
        } else if let Some(ref outer) = self.enclosure {
            outer.get(name)
        } else {
            Err(format!("Runtime Error: Undefined variable '{}'.", name))
        }
    }

    pub fn assign(&mut self, name: String, value: MistValue) -> Result<(), String> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else if let Some(ref mut outer) = self.enclosure {
            outer.assign(name, value)
        } else {
            Err(format!("Runtime Error: Undefined variable '{}'.", name))
        }
    }
}