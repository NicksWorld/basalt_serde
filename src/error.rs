use ::std::fmt::{self, Display};

use ::serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    InvalidData,
    Unimplemented,
    Eof,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::InvalidData => formatter.write_str("invalid data was received"),
            Error::Unimplemented => formatter.write_str("the datatype is not implemented"),
            Error::Eof => formatter.write_str("unexpected end of input"),
        }
    }
}

impl std::error::Error for Error {}
