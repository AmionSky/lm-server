use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug, Default)]
pub struct LmSrvError {
    pub module: &'static str,
    pub message: String,
}

impl LmSrvError {
    pub fn new(module: &'static str, message: String) -> Self {
        LmSrvError { module, message }
    }
}

impl Display for LmSrvError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "LmSrvError: {}: {}", self.module, self.message)
    }
}

impl Error for LmSrvError {}
