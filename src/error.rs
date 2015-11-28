use std::{error, fmt, io, result};

use url;
use hyper;

/// Errors that can occur during authentication flow.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Url(url::ParseError),
    Hyper(hyper::Error),
    Todo,
}

/// Result type returned from authentication flow methods.
pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Url(ref err) => write!(f, "{}", err),
            Error::Hyper(ref err) => write!(f, "{}", err),
            Error::Todo => write!(f, "Not implemented!"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "OAuth2 IO error",
            Error::Url(_) => "OAuth2 URL error",
            Error::Hyper(_) => "OAuth2 Hyper error",
            Error::Todo => "OAuth2 not implemented error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Url(ref err) => Some(err),
            Error::Hyper(ref err) => Some(err),
            _ => None,
        }
    }
}

macro_rules! impl_from {
    ($v:path, $t:ty) => {
        impl From<$t> for Error {
            fn from(err: $t) -> Error {
                $v(err)
            }
        }
    }
}

impl_from!(Error::Io, io::Error);
impl_from!(Error::Url, url::ParseError);
impl_from!(Error::Hyper, hyper::Error);
