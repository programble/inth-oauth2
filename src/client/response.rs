//! Response parsing.

use std::error::Error;
use std::fmt;

use rustc_serialize::json::Json;

/// Response parsing.
pub trait FromResponse: Sized {
    /// Parse a JSON response.
    fn from_response(json: &Json) -> Result<Self, ParseError>;
}

/// Response parse errors.
#[derive(Debug)]
pub enum ParseError {
    /// Expected response to be of type.
    ExpectedType(&'static str),

    /// Expected field to be of type.
    ExpectedFieldType(&'static str, &'static str),

    /// Expected field to equal value.
    ExpectedFieldValue(&'static str, &'static str),

    /// Expected field to not be present.
    UnexpectedField(&'static str),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ParseError::ExpectedType(t) =>
                write!(f, "Expected response of type {}", t),
            ParseError::ExpectedFieldType(k, t) =>
                write!(f, "Expected field {} of type {}", k, t),
            ParseError::ExpectedFieldValue(k, v) =>
                write!(f, "Expected field {} to equal {}", k, v),
            ParseError::UnexpectedField(k) =>
                write!(f, "Unexpected field {}", k),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str { "response parse error" }
}
