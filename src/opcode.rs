pub const OP_HALT: u8 = 0x00;

#[derive(Debug, Clone, Copy)]
pub enum OpCodeValue {
    Number(usize),
}
