#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Begin = 0b_0000_0000,
    End = 0b_0111_1111,

    Push = 0b_0000_0001,
}

#[derive(Debug, Clone, Copy)]
pub enum OpCodeValue {
    Number(usize),
}

pub fn read(token: &str) -> Option<OpCode> {
    // TODO: split token, create a type token splitted
    // TODO: continue from here
    let t: Vec<&str> = token.split('.').collect();
    match t[0].to_lowercase().as_str() {
        "begin" => Some(OpCode::Begin),
        "end" => Some(OpCode::End),
        "push" => Some(OpCode::Push),
        _ => None,
    }
}
