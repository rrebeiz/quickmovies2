use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;

#[derive(Debug)]
pub struct Validator {
    pub errors: HashMap<String, String>,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            errors: HashMap::new(),
        }
    }

    fn add_error(&mut self, key: &str, value: &str) {
        if !self.errors.contains_key(key) {
            self.errors.insert(key.to_string(), value.to_string());
        }
    }

    pub fn check(&mut self, ok: bool, key: &str, value: &str) {
        if !ok {
            self.add_error(key, value);
        }
    }
    pub fn valid(&self) -> bool {
        self.errors.is_empty()
    }
    pub fn permitted_value<T: PartialEq>(&self, items: &Vec<T>, permitted_values: Vec<T>) -> bool {
        items.iter().all(|item| permitted_values.contains(item))
    }

    // Fix this function
    pub fn unique<T: Eq + Hash + Clone>(&self, values: Vec<T>) -> bool {
        let mut unique_values: HashMap<T, bool> = HashMap::new();
        for value in values.iter() {
            unique_values.insert(value.clone(), true);
        }

        unique_values.len() == values.len()
    }
}
