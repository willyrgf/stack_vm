use crate::table::Table;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct WriteOnceTable<T>(HashMap<String, T>);

impl<T> WriteOnceTable<T> {
    pub fn new() -> Self {
        WriteOnceTable(HashMap::new())
    }

    fn already_exists_guard(&self, name: &str) {
        if self.0.contains_key(name) {
            panic!("Error: update constant {} now allowed", name);
        }
    }

    pub fn insert(&mut self, name: &str, value: T) {
        self.already_exists_guard(name);
        let name = String::from(name);
        self.0.insert(name, value);
    }

    pub fn keys(&self) -> Vec<String> {
        let mut result = vec![];
        self.0.keys().for_each(|key| result.push(key.to_string()));
        result
    }

    pub fn get(&self, op_name: &str) -> Option<&T> {
        self.0.get(op_name)
    }
}
