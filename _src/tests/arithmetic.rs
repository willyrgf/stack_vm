use crate::{
    assembler::Assembler, code::Code, instruction::Instruction, instruction_table::InstructionTable,
};

use super::super::machine::Machine;
use std::f64;

fn push(machine: &mut Machine<f64>, args: &[usize]) {
    let arg = *machine.get_data(args[0]);
    machine.operand_push(arg);
}

fn add(machine: &mut Machine<f64>, _args: &[usize]) {
    let rhs = machine.operand_pop();
    let lhs = machine.operand_pop();
    machine.operand_push(rhs + lhs);
}

fn instruction_table() -> InstructionTable<f64> {
    let mut it = InstructionTable::new();
    it.insert(Instruction::new(0, "push", 1, push));
    it.insert(Instruction::new(1, "add", 0, add));
    it
}

#[test]
fn addition_example() {
    let it = instruction_table();
    let mut assembler = Assembler::new(&it);

    assembler.push("push", vec![2.0]);
    assembler.push("push", vec![3.0]);
    assembler.push("add", vec![]);

    let code = Code::from(assembler);

    let mut machine = Machine::new(code, &it);
    machine.run();

    let result = machine.operand_pop();
    assert_eq!(result, 5.0);
}
