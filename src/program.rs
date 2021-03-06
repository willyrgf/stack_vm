use std::ops::Add;

use crate::{
    compiler,
    opcode::{self, Value},
};

const STACK_LIMIT: usize = 512;
const STACK_BEGIN: usize = 0;

macro_rules! binary_op {
    ($stack:expr, $v1:expr, $v2:expr, $op:tt) => {
        $stack.push(Value::Number($v1 $op $v2))
    };
}

macro_rules! comparison_op {
    ($stack:expr, $v1:expr, $v2:expr, $op:tt) => {
        let b = if $v1 $op $v2 { true } else { false };
        $stack.push(Value::from(b))
    };
}

#[derive(Debug)]
pub struct Program {
    ip: usize,
    sp: usize,
    code: Vec<u8>,
    constants: Vec<Value>,
    stack: Vec<Value>,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        Self {
            ip: 0,
            sp: 0,
            code: vec![],
            constants: vec![],
            stack: vec![],
        }
    }

    pub fn push(&mut self, value: Value) {
        if self.sp >= STACK_LIMIT {
            //TODO: create a macro to todo that
            log::error!(
                "push(): stack overflow, self.sp: {}, STACK_LIMIT: {}",
                self.sp,
                STACK_LIMIT
            );
            panic!()
        }

        self.stack.push(value);
        self.sp += 1;
    }

    pub fn pop(&mut self) -> Value {
        if self.sp == STACK_BEGIN {
            //TODO: create a macro to todo that
            log::error!(
                "pop(): empty stack, self.sp: {}, STACK_BEGIN: {}",
                self.sp,
                STACK_BEGIN
            );
            panic!()
        }

        self.sp -= 1;
        self.stack.pop().unwrap()
    }

    fn read_opcode(&mut self) -> u8 {
        let t = self.code[self.ip];
        self.ip += 1;
        t
    }

    pub fn exec(&mut self, program_code: &str) -> Value {
        //TODO: implement a s-expression parser in another crate
        let s_expressions = sexp::parse(program_code).unwrap();

        let compiled_code = compiler::compile(s_expressions);

        self.constants = compiled_code.constants();
        self.code = compiled_code.code();

        // let v = vec![
        //     Value::String("Hello, ".to_string()),
        //     Value::String("World".to_string()),
        // ];
        // // self.constants.push(Value::String("Hello, ".to_string()));
        // // self.constants.push(Value::String("World".to_string()));

        // let c = vec![
        //     opcode::OP_CONST,
        //     0,
        //     opcode::OP_CONST,
        //     1,
        //     opcode::OP_ADD,
        //     opcode::OP_HALT,
        // ];

        // log::debug!("program(): c expectd: {:?}", c);
        // log::debug!("program(): v expectd: {:?}", v);
        // log::debug!("program(): self.code: {:?}", self.code);
        // log::debug!("program(): self.constants: {:?}", self.constants);

        self.eval()
    }

    pub fn eval(&mut self) -> Value {
        loop {
            match self.read_opcode() {
                opcode::OP_HALT => {
                    log::debug!("eval(): OP_HALT, program: {:?}", self);
                    return self.pop();
                }
                opcode::OP_CONST => {
                    log::debug!("eval(): OP_CONST, program: {:?}", self.code);

                    let index = self.read_opcode() as usize;
                    log::debug!(
                        "eval(): OP_CONST, index: {:?}, constants: {:?}",
                        index,
                        self.constants
                    );

                    let value = self.constants[index].clone();
                    self.push(value.clone());

                    log::debug!(
                        "eval(): OP_CONST, sp: {:?}, value: {:?}, stack: {:?}",
                        self.sp,
                        value,
                        self.stack
                    );
                }
                opcode::OP_ADD => {
                    log::debug!("eval(): OP_ADD, program: {:?}", self.code);

                    let value2 = self.pop();
                    let value1 = self.pop();

                    //TODO: improve this code
                    if opcode::are_values_numbers(vec![&value1, &value2]) {
                        binary_op!(self, value1.as_number(), value2.as_number(), +);
                    } else if opcode::are_values_strings(vec![&value1, &value2]) {
                        let s = value1.as_string().add(&value2.as_string());
                        self.push(Value::String(s))
                    } else {
                        log::debug!("value1: {:?}, value2: {:?}", value1, value2);
                        log::error!("eval(): OP_ADD, this operation only supports: Value::Number, Value::String");
                        panic!()
                    }
                }
                opcode::OP_SUB => {
                    log::debug!("eval(): OP_SUB, program: {:?}", self.code);

                    let value2 = self.pop();
                    let value1 = self.pop();

                    if !(opcode::are_values_numbers(vec![&value1, &value2])) {
                        log::error!("eval(): OP_SUB, this operation only supports: Value::Number");
                        panic!()
                    }

                    binary_op!(self, value1.as_number(), value2.as_number(), -)
                }
                opcode::OP_MUL => {
                    log::debug!("eval(): OP_MUL, program: {:?}", self.code);

                    let value2 = self.pop();
                    let value1 = self.pop();

                    if !(opcode::are_values_numbers(vec![&value1, &value2])) {
                        log::error!("eval(): OP_MUL, this operation only supports: Value::Number");
                        panic!()
                    }

                    binary_op!(self, value1.as_number(), value2.as_number(), *)
                }
                opcode::OP_DIV => {
                    log::debug!("eval(): OP_DIV, program: {:?}", self.code);

                    let value2 = self.pop();
                    let value1 = self.pop();

                    if !(opcode::are_values_numbers(vec![&value1, &value2])) {
                        log::error!("eval(): OP_DIV, this operation only supports: Value::Number");
                        panic!()
                    }

                    binary_op!(self, value1.as_number(), value2.as_number(), /)
                }
                opcode::OP_EQ => {
                    log::debug!("eval(): OP_EQ, program: {:?}", self.code);

                    let value2 = self.pop();
                    let value1 = self.pop();

                    //TODO: remove this if chain to use tuples match
                    if opcode::are_values_numbers(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_number(), value2.as_number(), ==);
                    } else if opcode::are_values_strings(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_string(), value2.as_string(), ==);
                    } else if opcode::are_values_bools(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_bool(), value2.as_bool(), ==);
                    } else {
                        log::debug!("value1: {:?}, value2: {:?}", value1, value2);
                        log::error!("eval(): OP_EQ, this operation only supports: Value::Number, Value::String, Value::Bool");
                        panic!()
                    }
                }
                opcode::OP_NE => {
                    log::debug!("eval(): OP_EQ, program: {:?}", self.code);

                    let value2 = self.pop();
                    let value1 = self.pop();

                    //TODO: remove this if chain to use tuples match
                    if opcode::are_values_numbers(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_number(), value2.as_number(), !=);
                    } else if opcode::are_values_strings(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_string(), value2.as_string(), !=);
                    } else if opcode::are_values_bools(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_bool(), value2.as_bool(), !=);
                    } else {
                        log::debug!("value1: {:?}, value2: {:?}", value1, value2);
                        log::error!("eval(): OP_EQ, this operation only supports: Value::Number, Value::String, Value::Bool");
                        panic!()
                    }
                }
                opcode::OP_GT => {
                    log::debug!("eval(): OP_EQ, program: {:?}", self.code);

                    let value2 = self.pop();
                    let value1 = self.pop();

                    //TODO: remove this if chain to use tuples match
                    if opcode::are_values_numbers(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_number(), value2.as_number(), >);
                    } else if opcode::are_values_strings(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_string(), value2.as_string(), >);
                    } else if opcode::are_values_bools(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_bool(), value2.as_bool(), >);
                    } else {
                        log::debug!("value1: {:?}, value2: {:?}", value1, value2);
                        log::error!("eval(): OP_EQ, this operation only supports: Value::Number, Value::String, Value::Bool");
                        panic!()
                    }
                }
                opcode::OP_LT => {
                    log::debug!("eval(): OP_EQ, program: {:?}", self.code);

                    let value2 = self.pop();
                    let value1 = self.pop();

                    //TODO: remove this if chain to use tuples match
                    if opcode::are_values_numbers(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_number(), value2.as_number(), <);
                    } else if opcode::are_values_strings(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_string(), value2.as_string(), <);
                    } else if opcode::are_values_bools(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_bool(), value2.as_bool(), <);
                    } else {
                        log::debug!("value1: {:?}, value2: {:?}", value1, value2);
                        log::error!("eval(): OP_EQ, this operation only supports: Value::Number, Value::String, Value::Bool");
                        panic!()
                    }
                }
                opcode::OP_GE => {
                    log::debug!("eval(): OP_EQ, program: {:?}", self.code);

                    let value2 = self.pop();
                    let value1 = self.pop();

                    //TODO: remove this if chain to use tuples match
                    if opcode::are_values_numbers(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_number(), value2.as_number(), >=);
                    } else if opcode::are_values_strings(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_string(), value2.as_string(), >=);
                    } else if opcode::are_values_bools(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_bool(), value2.as_bool(), >=);
                    } else {
                        log::debug!("value1: {:?}, value2: {:?}", value1, value2);
                        log::error!("eval(): OP_EQ, this operation only supports: Value::Number, Value::String, Value::Bool");
                        panic!()
                    }
                }
                opcode::OP_LE => {
                    log::debug!("eval(): OP_EQ, program: {:?}", self.code);

                    let value2 = self.pop();
                    let value1 = self.pop();

                    //TODO: remove this if chain to use tuples match
                    if opcode::are_values_numbers(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_number(), value2.as_number(), <=);
                    } else if opcode::are_values_strings(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_string(), value2.as_string(), <=);
                    } else if opcode::are_values_bools(vec![&value1, &value2]) {
                        comparison_op!(self, value1.as_bool(), value2.as_bool(), <=);
                    } else {
                        log::debug!("value1: {:?}, value2: {:?}", value1, value2);
                        log::error!("eval(): OP_EQ, this operation only supports: Value::Number, Value::String, Value::Bool");
                        panic!()
                    }
                }
                _ => {
                    log::error!("eval(): opcode doesnt exist, program: {:?}", self);
                    panic!()
                }
            }
        }
    }
}
