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
        let name = String::from(name);
        self.0.insert(name, value);
    }
}
