use crate::instruction::Instruction;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Default)]
pub struct InstructionTable<T: fmt::Debug>(HashMap<usize, Instruction<T>>);
