use crate::{instruction_table::InstructionTable, table::Table, write_once_table::WriteOnceTable};
use std::fmt;

pub struct Assembler<'a, T: 'a + fmt::Debug + PartialEq> {
    pub instruction_table: &'a InstructionTable<T>,
    pub instructions: Vec<usize>,
    pub labels: WriteOnceTable<usize>,
    pub data: Vec<T>,
}

impl<'a, T: 'a + fmt::Debug + PartialEq> Assembler<'a, T> {
    pub fn new(it: &'a InstructionTable<T>) -> Self {
        let mut labels: WriteOnceTable<usize> = WriteOnceTable::new();
        labels.insert("main", 0);

        Assembler {
            instruction_table: it,
            instructions: vec![],
            labels,
            data: vec![],
        }
    }

    pub fn push_data(&mut self, data: T) -> usize {
        let pos = self.data.iter().position(|d| d == &data);
        match pos {
            Some(pos) => pos,
            None => {
                self.data.push(data);
                self.data.len() - 1
            }
        }
    }

    pub fn push(&mut self, op_name: &str, args: Vec<T>) {
        let instruction = self
            .instruction_table
            .by_op_name(op_name)
            .unwrap_or_else(|| panic!("Error: the op_name {} doesnt exist", op_name));

        if args.len() != instruction.arity {
            panic!(
                "Error: instruction {} has arity of {}, but provided {} args",
                op_name,
                instruction.arity,
                args.len(),
            );
        }

        self.instructions.push(instruction.op_code);
        self.instructions.push(instruction.arity);

        for arg in args {
            let pos = self.push_data(arg);
            self.instructions.push(pos);
        }
    }

    pub fn label(&mut self, op_name: &str) {
        self.labels.insert(op_name, self.instructions.len());
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instructions.is_empty()
    }
}
