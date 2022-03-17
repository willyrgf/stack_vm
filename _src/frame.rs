use crate::table::Table;
use crate::write_many_table::WriteManyTable;

#[derive(Debug)]
pub struct Frame<T> {
    locals: WriteManyTable<T>,
    pub return_address: usize,
}
