use std::error::Error;
use std::{fmt, io};

use hyper;
use serde_json;
use url;

use client::response::ParseError;
use error::OAuth2Error;

/// Errors that can occur during authorization.
#[derive(Debug)]
pub enum ClientError {
    /// IO error.
    Io(io::Error),

    /// URL error.
    Url(url::ParseError),

    /// Hyper error.
    Hyper(hyper::Error),

    /// JSON error.
    Json(serde_json::Error),

    /// Response parse error.
    Parse(ParseError),

    /// OAuth 2.0 error.
    OAuth2(OAuth2Error),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ClientError::Io(ref err) => write!(f, "{}", err),
            ClientError::Url(ref err) => write!(f, "{}", err),
            ClientError::Hyper(ref err) => write!(f, "{}", err),
            ClientError::Json(ref err) => write!(f, "{}", err),
            ClientError::Parse(ref err) => write!(f, "{}", err),
            ClientError::OAuth2(ref err) => write!(f, "{}", err),
        }
    }
}

impl Error for ClientError {
    fn description(&self) -> &str {
        match *self {
            ClientError::Io(ref err) => err.description(),
            ClientError::Url(ref err) => err.description(),
            ClientError::Hyper(ref err) => err.description(),
            ClientError::Json(ref err) => err.description(),
            ClientError::Parse(ref err) => err.description(),
            ClientError::OAuth2(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ClientError::Io(ref err) => Some(err),
            ClientError::Url(ref err) => Some(err),
            ClientError::Hyper(ref err) => Some(err),
            ClientError::Json(ref err) => Some(err),
            ClientError::Parse(ref err) => Some(err),
            ClientError::OAuth2(ref err) => Some(err),
        }
    }
}

macro_rules! impl_from {
    ($v:path, $t:ty) => {
        impl From<$t> for ClientError {
            fn from(err: $t) -> Self {
                $v(err)
            }
        }
    }
}

impl_from!(ClientError::Io, io::Error);
impl_from!(ClientError::Url, url::ParseError);
impl_from!(ClientError::Hyper, hyper::Error);
impl_from!(ClientError::Json, serde_json::Error);
impl_from!(ClientError::Parse, ParseError);
impl_from!(ClientError::OAuth2, OAuth2Error);
