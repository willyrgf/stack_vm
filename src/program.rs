use crate::opcode::{self, Value};
use core::panic;

const STACK_LIMIT: usize = 512;
const STACK_BEGIN: usize = 0;

#[derive(Debug)]
pub struct Program {
    ip: usize,
    sp: usize,
    code: Vec<u8>,
    constants: Vec<Value>,
    stack: [Value; STACK_LIMIT],
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
            stack: [Value::None; STACK_LIMIT],
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

        self.stack[self.sp] = value;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> Value {
        if self.sp == STACK_BEGIN {
            //TODO: create a macro to todo that
            log::error!(
                "push(): empty stack, self.sp: {}, STACK_BEGIN: {}",
                self.sp,
                STACK_BEGIN
            );
            panic!()
        }

        self.sp -= 1;

        self.stack[self.sp]
    }

    fn read_opcode(&mut self) -> u8 {
        let t = self.code[self.ip];
        self.ip += 1;
        t
    }

    pub fn exec(&mut self, _program: &str) -> Value {
        // 1. parser the program
        // let ast = parser(program);
        // 2. compile the program to bytecode
        // let code = compile(ast);

        self.constants.push(Value::Number(2_f64));
        self.constants.push(Value::Number(3_f64));

        self.code = vec![
            opcode::OP_CONST,
            0,
            opcode::OP_CONST,
            1,
            opcode::OP_ADD,
            opcode::OP_HALT,
        ];

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

                    let value = self.constants[index];
                    self.push(value);

                    log::debug!(
                        "eval(): OP_CONST, sp: {:?}, value: {:?}, stack: {:?}",
                        value,
                        self.sp,
                        self.stack
                    );
                }

                opcode::OP_ADD => {
                    log::debug!("eval(): OP_CONST, program: {:?}", self.code);

                    //TODO: add macro to binary operation
                    let op1 = self.pop().as_number();
                    let op2 = self.pop().as_number();

                    let result = op1 + op2;

                    self.push(Value::Number(result));
                }
                _ => {
                    log::error!("eval(): opcode doesnt exist, program: {:?}", self);
                    panic!()
                }
            }
        }
    }
}
