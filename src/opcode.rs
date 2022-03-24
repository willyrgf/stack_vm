pub const OP_HALT: u8 = 0x00;
pub const OP_CONST: u8 = 0x01;
pub const OP_ADD: u8 = 0x02;

#[derive(Debug, Clone, Copy)]
// A Value for the stack. (tagged union)
pub enum Value {
    None,
    Number(f64),
}

impl Value {
    pub fn as_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n as f64,
            Value::None => 0_f64,
        }
    }
}
