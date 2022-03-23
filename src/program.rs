use crate::opcode::{self, OpCode, OpCodeValue};

#[derive(Debug)]
pub struct Program<'a> {
    ip: usize,
    code: Vec<&'a str>,
    stack: Vec<OpCodeValue>,
}

impl<'a> Program<'a> {
    pub fn new() -> Self {
        Self {
            ip: 0,
            code: vec![],
            stack: vec![],
        }
    }

    fn read_opcode(&mut self) -> Option<OpCode> {
        let t = self.code[self.ip];
        self.ip += 1;
        opcode::read(t)
    }

    pub fn exec(&mut self, _program: &str) {
        // 1. parser the program
        // let ast = parser(program);
        // 2. compile the program to bytecode
        // let code = compile(ast);

        let script = "begin push.1 end";

        self.code = script.split_whitespace().collect();

        self.ip = 0;

        self.eval()
    }

    pub fn eval(&mut self) {
        loop {
            match self.read_opcode().unwrap() {
                OpCode::Push => {
                    log::debug!("eval(): Push, program: {:?}", self);
                    // self.stack.push(value)
                    return;
                }
                OpCode::Begin => {
                    log::debug!("eval(): Begin, program: {:?}", self);
                    return;
                }
                OpCode::End => {
                    log::debug!("eval(): End, program: {:?}", self);
                    return;
                }
            }
        }
    }
}
