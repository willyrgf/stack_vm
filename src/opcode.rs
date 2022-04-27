pub const OP_HALT: u8 = 0x00;
pub const OP_CONST: u8 = 0x01;
pub const OP_ADD: u8 = 0x02;
pub const OP_SUB: u8 = 0x03;
pub const OP_MUL: u8 = 0x04;
pub const OP_DIV: u8 = 0x05;
pub const OP_EQ: u8 = 0x06;
pub const OP_NE: u8 = 0x07;
pub const OP_GE: u8 = 0x08;
pub const OP_LE: u8 = 0x09;
pub const OP_GT: u8 = 0x10;
pub const OP_LT: u8 = 0x11;
pub const OP_TRUE: u8 = 0x11;

//TODO: add a OP to true and false?

#[derive(Debug, Clone)]
// A Value for the stack. (tagged union)
pub enum Value {
    None,
    Number(f64),
    String(String),
    Bool(bool),
}

macro_rules! into_raw {
    ($value:expr, $expected_type:path) => {
        match $value {
            $expected_type(v) => Some(v.clone()),
            _ => None,
        }
    };
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Number(i as f64)
    }
}

impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Number(f)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        match s.as_str() {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => Value::String(s),
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl Value {
    pub fn is_number(&self) -> bool {
        self.try_as_number().is_some()
    }

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

    pub fn is_string(&self) -> bool {
        self.try_as_string().is_some()
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

    pub fn is_bool(&self) -> bool {
        self.try_as_bool().is_some()
    }

    pub fn try_as_bool(&self) -> Option<bool> {
        into_raw!(self, Self::Bool)
    }

    pub fn as_bool(&self) -> bool {
        match self.try_as_bool() {
            Some(v) => v,
            None => {
                log::error!(
                    "as_bool!(): value must be a opcode::Value::Bool, value: {:?}",
                    self,
                );
                panic!()
            }
        }
    }
}

pub fn are_values_numbers(values: Vec<&Value>) -> bool {
    values.into_iter().all(|v| v.is_number())
}

pub fn are_values_strings(values: Vec<&Value>) -> bool {
    values.into_iter().all(|v| v.is_string())
}

pub fn are_values_bools(values: Vec<&Value>) -> bool {
    values.into_iter().all(|v| v.is_bool())
}
