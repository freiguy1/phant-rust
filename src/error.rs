use hyper::error::Error as HyperError;
use hyper::error::UriError;
use std::io::Error as IoError;
use std::error::Error as StdError;
use serde_json::Error as JsonError;
use native_tls::Error as NativeTlsError;

#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    Uri(UriError),
    Io(IoError),
    Serde(JsonError),
    NativeTls(NativeTlsError),
    Phant(String),
}

impl From<HyperError> for Error {
    fn from(e: HyperError) -> Error {
        Error::Hyper(e)
    }
}

impl From<UriError> for Error {
    fn from(e: UriError) -> Error {
        Error::Uri(e)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Error {
        Error::Io(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Error {
        Error::Serde(e)
    }
}

impl From<NativeTlsError> for Error {
    fn from(e: NativeTlsError) -> Error {
        Error::NativeTls(e)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Hyper(ref e) => e.description(),
            Error::Uri(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::Serde(ref e) => e.description(),
            Error::NativeTls(ref e) => e.description(),
            Error::Phant(ref message) => &message
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Hyper(ref e) => Some(e),
            Error::Uri(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
            Error::Serde(ref e) => Some(e),
            Error::NativeTls(ref e) => Some(e),
            Error::Phant(_) => None
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(),::std::fmt::Error> {
        f.write_str(self.description())
    }
}
