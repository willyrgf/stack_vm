use crate::code::Code;
use crate::frame::Frame;
use crate::instruction_table::InstructionTable;
use crate::stack::Stack;
use crate::table::Table;
use std::fmt;

pub struct Machine<'a, T: 'a + fmt::Debug> {
    pub code: Code<T>,
    pub instruction_table: &'a InstructionTable<T>,
    pub ip: usize,
    pub constants: &'a dyn Table<Item = T>,
    pub call_stack: Stack<Frame<T>>,
    pub operand_stack: Stack<T>,
}
