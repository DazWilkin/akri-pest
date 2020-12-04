use std::{error, fmt};
#[derive(Clone, Debug, PartialEq)]
pub struct ParseError {
    message: String,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error")
    }
}
impl error::Error for ParseError {}
impl ParseError {
    pub fn new(e: &str) -> ParseError {
        ParseError {
            message: e.to_string(),
        }
    }
}
