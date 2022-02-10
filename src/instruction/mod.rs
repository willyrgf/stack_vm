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

impl<T: fmt::Debug> Instruction<T> {
    fn new(op_code: usize, op_name: &str, arity: usize, fun: InstructionFn<T>) -> Self {
        Instruction {
            op_code,
            op_name: String::from(op_name),
            arity,
            fun,
        }
    }
}

mod test {
    use super::Instruction;
    use crate::machine::Machine;

    #[derive(Debug)]
    struct Operand(i64);

    fn noop(_machine: &mut Machine<Operand>, _args: &[usize]) {}

    #[test]
    fn new() {
        let operand = Instruction::new(7, "noop", 3, noop);
        assert_eq!(operand.op_code, 7);
        assert_eq!(operand.op_name, "noop".to_string());
        assert_eq!(operand.arity, 3);
    }
}
