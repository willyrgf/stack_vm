pub const OP_HALT: u8 = 0x00;
pub const OP_CONST: u8 = 0x01;

#[derive(Debug, Clone, Copy)]
// A Value for the stack. (tagged union)
pub enum Value {
    None,
    Number(u8),
}
