use crate::code::Code;
use crate::frame::Frame;
use crate::instruction_table::{self, InstructionTable};
use crate::stack::Stack;
use crate::table::Table;
use std::fmt;

pub struct Machine<'a, T: 'a + fmt::Debug> {
    pub code: Code<T>,
    pub instruction_table: &'a InstructionTable<T>,
    pub ip: usize,
    pub operand_stack: Stack<T>,
}

impl<'a, T: 'a + fmt::Debug> Machine<'a, T> {
    pub fn new(code: Code<T>, instruction_table: &'a InstructionTable<T>) -> Self {
        Machine {
            code,
            instruction_table,
            ip: 0,
            operand_stack: Stack::new(),
        }
    }

    fn next_code(&mut self) -> usize {
        let code = self.code.code[self.ip];
        self.ip = self.ip + 1;
        code
    }

    pub fn run(&mut self) {
        loop {
            if self.ip >= self.code.code.len() {
                break;
            }

            let op_code = self.next_code();
            let arity = self.next_code();

            let instruction = self
                .instruction_table
                .by_op_code(op_code)
                .unwrap_or_else(|| panic!("Error: unable to find op code {}", op_code));

            let mut args = vec![];
            for _ in 0..arity {
                args.push(self.next_code());
            }

            let fun = instruction.fun;
            fun(self, args.as_slice());
        }
    }

    pub fn operand_push(&mut self, value: T) {
        self.operand_stack.push(value)
    }

    pub fn operand_pop(&mut self) -> T {
        self.operand_stack.pop()
    }
}
