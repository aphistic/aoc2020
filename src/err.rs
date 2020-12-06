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