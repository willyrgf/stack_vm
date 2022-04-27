use crate::opcode::{self, Value};

macro_rules! add_const_with_value {
    ($cc:expr, $v1:expr) => {
        $cc.code.push(opcode::OP_CONST);
        $cc.code.push($cc.constants.len().try_into().unwrap());
        $cc.constants.push(Value::from($v1));
    };
}

#[derive(Debug, Default)]
pub struct CompiledCode {
    code: Vec<u8>,
    constants: Vec<Value>,
}

fn symbol_to_opcode(s: sexp::Sexp) -> Option<u8> {
    match s {
        sexp::Sexp::Atom(sexp::Atom::S(s)) => match s.as_str() {
            "+" => Some(opcode::OP_ADD),
            "-" => Some(opcode::OP_SUB),
            "*" => Some(opcode::OP_MUL),
            "/" => Some(opcode::OP_DIV),
            "==" => Some(opcode::OP_EQ),
            "!=" => Some(opcode::OP_NE),
            ">" => Some(opcode::OP_GT),
            "<" => Some(opcode::OP_LT),
            ">=" => Some(opcode::OP_GE),
            "<=" => Some(opcode::OP_LE),
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

                match symbol_to_opcode(e.clone()) {
                    Some(op) => {
                        self.gen(l[1].clone());
                        self.gen(l[2].clone());
                        self.code.push(op);
                    }
                    _ => self.gen(e),
                }
            }
            sexp::Sexp::Atom(a) => match a {
                sexp::Atom::I(i) => {
                    add_const_with_value!(self, i);
                }
                sexp::Atom::F(f) => {
                    add_const_with_value!(self, f);
                }
                sexp::Atom::S(s) => {
                    add_const_with_value!(self, s);
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

    log::debug!("compile(): compiled_code: {:?}", cc);
    cc
}
