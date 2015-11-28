use std::result;

use url;
use hyper;

/// Errors that can occur during authentication flow.
#[derive(Debug)]
pub enum Error {
    Url(url::ParseError),
    Hyper(hyper::Error),
}

/// Result type returned from authentication flow methods.
pub type Result<T> = result::Result<T, Error>;

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::Url(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}
