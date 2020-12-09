#[derive(PartialEq, Debug)]
pub struct ParseError {
    msg: String,
    data: String,
}

impl ParseError {
    pub fn new(msg: &str, data: &str) -> ParseError {
        ParseError {
            msg: msg.to_owned(),
            data: data.to_owned(),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ExecError {
    msg: String,
    pc: u32,
}

impl ExecError {
    pub fn new(msg: &str, pc: u32) -> ExecError {
        ExecError{
            msg: msg.to_owned(),
            pc: pc,
        }
    }
}