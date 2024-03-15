use std::fmt;
use pcsc::Error;

#[derive(Debug)]
pub enum ReaderError {
    UnsupportedReader(String),
    PcscError(Error),
    NoReadersFound,
}

impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReaderError::UnsupportedReader(ref reader) => write!(f, "Unsupported reader: {}", reader),
            ReaderError::PcscError(ref err) => write!(f, "PCSC error: {}", err),
            ReaderError::NoReadersFound => write!(f, "No readers found"),
        }
    }
}

impl From<Error> for ReaderError {
    fn from(err: Error) -> ReaderError {
        ReaderError::PcscError(err)
    }
}
