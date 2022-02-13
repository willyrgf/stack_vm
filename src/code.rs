use crate::assembler::{self, Assembler};
use std::fmt;

pub struct Code<T> {
    pub symbols: Vec<(usize, String)>,
    pub code: Vec<usize>,
    pub data: Vec<T>,
    pub labels: Vec<(usize, String)>,
}

impl<'a, T: 'a + fmt::Debug + PartialEq> From<Assembler<'a, T>> for Code<T> {
    fn from(assembler: Assembler<T>) -> Self {
        let symbols = assembler.instruction_table.symbols();
        let code = assembler.instructions;
        let data = assembler.data;
        let mut labels: Vec<(usize, String)> = vec![];

        for key in assembler.labels.keys() {
            let idx = assembler.labels.get(&key).unwrap();
            labels.push((*idx, key));
        }

        Code {
            symbols,
            code,
            data,
            labels,
        }
    }
}
