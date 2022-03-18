#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Begin = 0b_0000_0000,
    End = 0b_0111_1111,
}
