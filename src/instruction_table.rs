use crate::instruction::Instruction;
use std::collections::HashMap;
use std::fmt;

pub struct InstructionTable<T: fmt::Debug>(HashMap<usize, Instruction<T>>);

impl<T: fmt::Debug> InstructionTable<T> {
    pub fn new() -> Self {
        InstructionTable(HashMap::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn by_op_code(&self, op_code: usize) -> Option<&Instruction<T>> {
        self.0.get(&op_code)
    }

    pub fn by_op_name(&self, op_name: &str) -> Option<&Instruction<T>> {
        self.0
            .values()
            .find(|instruction| instruction.op_name == op_name)
    }

    pub fn insert(&mut self, instruction: Instruction<T>) {
        self.0.insert(instruction.op_code, instruction);
    }
}

mod test {
    use super::{HashMap, Instruction, InstructionTable};
    use crate::machine::Machine;

    #[derive(Debug)]
    struct Operand(i64);

    const OP_CODE: usize = 7;
    const OP_NAME: &str = "noop";
    const ARITY: usize = 3;

    fn noop(_machine: &mut Machine<Operand>, _args: &[usize]) {}

    #[test]
    fn new() {
        let it: InstructionTable<Operand> = InstructionTable::new();
        assert!(it.is_empty())
    }

    #[test]
    fn is_empty() {
        let it: InstructionTable<Operand> = InstructionTable::new();
        assert!(it.is_empty());
    }

    fn by_op_code() {
        let mut it: InstructionTable<Operand> = InstructionTable::new();
        it.insert(Instruction::new(OP_CODE, OP_NAME, ARITY, noop));
        let instruction = it.by_op_code(0).unwrap();
        assert_eq!(instruction.op_name, OP_NAME)
    }

    fn by_op_name() {
        let mut it: InstructionTable<Operand> = InstructionTable::new();
        it.insert(Instruction::new(OP_CODE, OP_NAME, ARITY, noop));
        let instruction = it.by_op_name(OP_NAME).unwrap();
        assert_eq!(instruction.op_code, OP_CODE);
    }

    fn insert() {
        let mut it: InstructionTable<Operand> = InstructionTable::new();
        it.insert(Instruction::new(OP_CODE, OP_NAME, ARITY, noop));
        assert!(!it.is_empty())
    }
}
