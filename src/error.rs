use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidCandle(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidCandle(msg) => write!(f, "Invalid candle: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
