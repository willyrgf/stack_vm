use crate::machine::Machine;
use std::fmt;

pub type InstructionFn<T> = fn(machine: &mut Machine<T>, args: &[usize]);

pub struct Instruction<T: fmt::Debug> {
    pub op_code: usize,
    pub op_name: String,
    pub arity: usize,
    pub fun: InstructionFn<T>,
}

impl<T: fmt::Debug> fmt::Debug for Instruction<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Instruction {{ op_code: {}, name: {}, arity: {} }}",
            self.op_code, self.op_name, self.arity
        )
    }
}
