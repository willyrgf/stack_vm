use crate::opcode::OpCode;

#[derive(Debug)]
pub struct Program {
    ip: usize,
    code: Vec<OpCode>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            ip: 0,
            code: vec![],
        }
    }

    fn read_opcode(&mut self) -> OpCode {
        let c = self.code[self.ip];
        self.ip += 1;
        c
    }

    pub fn exec(&mut self, _program: &str) {
        // 1. parser the program
        // let ast = parser(program);
        // 2. compile the program to bytecode
        // let code = compile(ast);

        self.code = vec![OpCode::End];
        self.ip = 0;

        self.eval()
    }

    pub fn eval(&mut self) {
        loop {
            match self.read_opcode() {
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
