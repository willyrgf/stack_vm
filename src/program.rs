use crate::opcode::{self, OpCodeValue};
use core::panic;

#[derive(Debug)]
pub struct Program {
    ip: usize,
    sp: usize,
    code: Vec<u8>,
    stack: Vec<OpCodeValue>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            ip: 0,
            sp: 0,
            code: vec![],
            stack: vec![],
        }
    }

    fn read_opcode(&mut self) -> u8 {
        let t = self.code[self.ip];
        self.ip += 1;
        t
    }

    pub fn exec(&mut self, _program: &str) {
        // 1. parser the program
        // let ast = parser(program);
        // 2. compile the program to bytecode
        // let code = compile(ast);

        self.code = vec![opcode::OP_HALT];

        self.eval()
    }

    pub fn eval(&mut self) {
        loop {
            match self.read_opcode() {
                opcode::OP_HALT => {
                    log::debug!("eval(): OP_HALT, program: {:?}", self);
                    return;
                }
                _ => {
                    log::error!("eval(): opcode doesnt exist, program: {:?}", self);
                    panic!()
                }
            }
        }
    }
}
