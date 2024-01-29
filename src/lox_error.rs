use std::fmt;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, LoxError>;

#[derive(Debug, Clone)]
pub struct LoxError;

impl Display for LoxError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error Occured")
    }
}

#[derive(Debug, Clone)]
pub struct LoxTokenizationError;

impl Display for LoxTokenizationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Unexpected character while parsing at line")
    }
}
