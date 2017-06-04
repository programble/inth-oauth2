//! Response parsing.

use std::error::Error;
use std::fmt;

use serde_json::Value;

/// Response parsing.
pub trait FromResponse: Sized {
    /// Parse a JSON response.
    fn from_response(json: &Value) -> Result<Self, ParseError>;

    /// Parse a JSON response, inheriting missing values from the previous instance.
    ///
    /// Necessary for parsing refresh token responses where the absence of a new refresh token
    /// implies that the previous refresh token is still valid.
    #[allow(unused_variables)]
    fn from_response_inherit(json: &Value, prev: &Self) -> Result<Self, ParseError> {
        FromResponse::from_response(json)
    }
}

/// Response parse errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
