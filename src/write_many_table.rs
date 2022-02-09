use crate::table::Table;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct WriteManyTable<T>(HashMap<String, T>);
