use crate::opcode::{self, Value};

#[derive(Debug, Default)]
pub struct CompiledCode {
    code: Vec<u8>,
    constants: Vec<Value>,
}

impl CompiledCode {
    pub fn code(&self) -> Vec<u8> {
        self.code.clone()
    }
    pub fn constants(&self) -> Vec<Value> {
        self.constants.clone()
    }

    fn gen(&mut self, expressions: sexp::Sexp, pos: u8) {
        match expressions {
            sexp::Sexp::List(l) => l.into_iter().for_each(|e| {
                // self.pos += 1;
                self.gen(e, pos + 1);
            }),
            sexp::Sexp::Atom(a) => match a {
                sexp::Atom::I(n) => {
                    self.code.push(opcode::OP_CONST);
                    self.code.push(self.constants.len() as u8);
                    self.constants.push(Value::Number(n as f64));
                }
                sexp::Atom::F(n) => {
                    self.code.push(opcode::OP_CONST);
                    self.code.push(self.constants.len() as u8);
                    self.constants.push(Value::Number(n));
                }
                sexp::Atom::S(s) => {
                    //TODO: handle with special characters like ops + - * /
                    match s.as_str() {
                        "+" => {
                            self.code.push(opcode::OP_ADD);
                        }
                        _ => {
                            self.code.push(opcode::OP_CONST);
                            self.code.push(self.constants.len() as u8);
                            self.constants.push(Value::String(s));
                        }
                    }
                }
            },
        }
    }
}

pub fn compile(expressions: sexp::Sexp) -> CompiledCode {
    let mut cc = CompiledCode::default();

    //TODO: implement a s-expression parser in another crate
    cc.gen(expressions, 0);
    cc.code.push(opcode::OP_HALT);

    log::debug!("compile(): cc: {:?}", cc);
    cc
}
