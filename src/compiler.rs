use crate::opcode::{self, Value};

#[derive(Debug, Default)]
pub struct CompiledCode {
    code: Vec<u8>,
    constants: Vec<Value>,
}

fn symbol(s: sexp::Sexp) -> Option<String> {
    match s {
        sexp::Sexp::Atom(sexp::Atom::S(s)) => match s.as_str() {
            "+" => Some("+".to_string()),
            "-" => Some("-".to_string()),
            "*" => Some("*".to_string()),
            "/" => Some("/".to_string()),
            _ => None,
        },
        _ => None,
    }
}

impl CompiledCode {
    pub fn code(&self) -> Vec<u8> {
        self.code.clone()
    }
    pub fn constants(&self) -> Vec<Value> {
        self.constants.clone()
    }

    fn gen(&mut self, expressions: sexp::Sexp) {
        match expressions {
            sexp::Sexp::List(l) => {
                let e = l[0].clone();

                match symbol(e.clone()) {
                    Some(symbol) => {
                        self.gen(l[1].clone());
                        self.gen(l[2].clone());
                        match symbol.as_str() {
                            "+" => self.code.push(opcode::OP_ADD),
                            "-" => self.code.push(opcode::OP_SUB),
                            "*" => self.code.push(opcode::OP_MUL),
                            "/" => self.code.push(opcode::OP_DIV),
                            _ => {
                                panic!("gen(): unreachable code");
                            }
                        }
                    }
                    _ => self.gen(e),
                }
            }
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
                    self.code.push(opcode::OP_CONST);
                    self.code.push(self.constants.len() as u8);
                    self.constants.push(Value::String(s));
                }
            },
        }
    }
}

pub fn compile(expressions: sexp::Sexp) -> CompiledCode {
    let mut cc = CompiledCode::default();

    //TODO: implement a s-expression parser in another crate
    cc.gen(expressions);
    cc.code.push(opcode::OP_HALT);

    log::debug!("compile(): cc: {:?}", cc);
    cc
}
