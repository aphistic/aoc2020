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

    pub fn msg(&self) -> &str {
        self.msg.as_str()
    }
    pub fn data(&self) -> &str {
        self.data.as_str()
    }
}