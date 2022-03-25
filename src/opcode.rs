pub const OP_HALT: u8 = 0x00;
pub const OP_CONST: u8 = 0x01;
pub const OP_ADD: u8 = 0x02;
pub const OP_SUB: u8 = 0x03;
pub const OP_MUL: u8 = 0x04;
pub const OP_DIV: u8 = 0x05;

#[derive(Debug, Clone)]
// A Value for the stack. (tagged union)
pub enum Value {
    None,
    Number(f64),
    String(String),
}

macro_rules! into_raw {
    ($value:expr, $expected_type:path) => {
        match $value {
            $expected_type(v) => Some(v.clone()),
            _ => None,
        }
    };
}

impl Value {
    pub fn try_as_number(&self) -> Option<f64> {
        into_raw!(self, Self::Number)
    }

    pub fn as_number(&self) -> f64 {
        match self.try_as_number() {
            Some(v) => v,
            None => {
                log::error!(
                    "as_number!(): value must be a opcode::Value::Number, value: {:?}",
                    self,
                );
                panic!()
            }
        }
    }

    pub fn try_as_string(&self) -> Option<String> {
        into_raw!(self, Self::String)
    }

    pub fn as_string(&self) -> String {
        match self.try_as_string() {
            Some(v) => v,
            None => {
                log::error!(
                    "as_string!(): value must be a opcode::Value::String, value: {:?}",
                    self,
                );
                panic!()
            }
        }
    }
}
