use hyper::error::Error as HyperError;
use std::io::Error as IoError;

pub struct Error {
    pub error: String
}

impl From<HyperError> for Error {
    fn from(e: HyperError) -> Error {
        Error { error: format!("{}", e) }
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error { error: format!("{}", e) }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(),::std::fmt::Error> {
        write!(f, "{}", self.error)
    }
}
