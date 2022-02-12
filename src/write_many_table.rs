use crate::table::Table;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct WriteManyTable<T>(HashMap<String, T>);

impl<T> WriteManyTable<T> {
    pub fn new() -> Self {
        WriteManyTable(HashMap::new())
    }

    pub fn insert(&mut self, name: &str, value: T) {
        let name = String::from(name);
        self.0.insert(name, value);
    }
}
