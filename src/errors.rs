use std::fmt;
use pcsc::{Error};

#[derive(Debug)]
pub enum ReaderError {
    UnsupportedReader(String),
    PcscError(Error),
    NoReadersFound,
    CardError(String, Error),
}


impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReaderError::UnsupportedReader(ref reader) => write!(f, "Unsupported reader: {}", reader),
            ReaderError::PcscError(ref err) => write!(f, "PCSC error: {}", err),
            ReaderError::NoReadersFound => write!(f, "No readers found"),
            ReaderError::CardError(ref card, ref err) => {
                // Assuming you decide to omit the Card details from the debug output
                write!(f, "Card error: {}, {}", "Card details omitted", err)
            },
        }
    }
}
impl From<Error> for ReaderError {
    fn from(err: Error) -> ReaderError {
        ReaderError::PcscError(err)
    }
}
