use std::{error, fmt, io, result};

use hyper;
use rustc_serialize::json;
use url;

/// OAuth 2.0 error codes.
#[derive(Debug)]
pub enum OAuth2ErrorCode {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,
    Unrecognized(String),
}

/// OAuth 2.0 error.
///
/// See [RFC6749](http://tools.ietf.org/html/rfc6749#section-5.2).
#[derive(Debug)]
pub struct OAuth2Error {
    pub code: OAuth2ErrorCode,
    pub description: Option<String>,
    pub uri: Option<String>,
}

impl fmt::Display for OAuth2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{:?}", self.code));
        if let Some(ref description) = self.description {
            try!(write!(f, ": {}", description));
        }
        if let Some(ref uri) = self.uri {
            try!(write!(f, " ({})", uri));
        }
        Ok(())
    }
}

impl error::Error for OAuth2Error {
    fn description(&self) -> &str {
        "OAuth2 API error"
    }
}

/// Errors that can occur during authentication flow.
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Url(url::ParseError),
    Hyper(hyper::Error),
    Json(json::DecoderError),
    OAuth2(OAuth2Error),
}

/// Result type returned from authentication flow methods.
pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "{}", err),
            Error::Url(ref err) => write!(f, "{}", err),
            Error::Hyper(ref err) => write!(f, "{}", err),
            Error::Json(ref err) => write!(f, "{}", err),
            Error::OAuth2(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "OAuth2 IO error",
            Error::Url(_) => "OAuth2 URL error",
            Error::Hyper(_) => "OAuth2 Hyper error",
            Error::Json(_) => "OAuth2 JSON error",
            Error::OAuth2(_) => "OAuth2 API error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Url(ref err) => Some(err),
            Error::Hyper(ref err) => Some(err),
            Error::Json(ref err) => Some(err),
            Error::OAuth2(ref err) => Some(err),
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
impl_from!(Error::Json, json::DecoderError);
impl_from!(Error::OAuth2, OAuth2Error);
