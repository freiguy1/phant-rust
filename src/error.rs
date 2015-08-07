use hyper::error::Error as HyperError;
use std::io::Error as IoError;
use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    Io(IoError),
    Phant(String),
}

impl From<HyperError> for Error {
    fn from(e: HyperError) -> Error {
        Error::Hyper(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::Io(e)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::Phant(ref message) => &message
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Hyper(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
            Error::Phant(_) => None
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(),::std::fmt::Error> {
        f.write_str(self.description())
    }
}
