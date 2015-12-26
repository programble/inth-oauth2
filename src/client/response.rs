//! Response parsing.

use std::error::Error;
use std::fmt;

use rustc_serialize::json::{self, Json};

/// Response parsing.
pub trait FromResponse: Sized {
    /// Parse a JSON response.
    fn from_response(json: &Json) -> Result<Self, ParseError>;

    /// Parse a JSON response, inheriting missing values from the previous instance.
    ///
    /// Necessary for parsing refresh token responses where the absence of a new refresh token
    /// implies that the previous refresh token is still valid.
    #[allow(unused_variables)]
    fn from_response_inherit(json: &Json, prev: &Self) -> Result<Self, ParseError> {
        FromResponse::from_response(json)
    }
}

/// Response parse errors.
#[derive(Debug, PartialEq, Eq)]
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

/// JSON helper for response parsing.
#[derive(Debug)]
pub struct JsonHelper<'a>(pub &'a Json);

impl<'a> JsonHelper<'a> {
    /// Returns self as a `JsonObjectHelper` or fails with `ParseError::ExpectedType`.
    pub fn as_object(&self) -> Result<JsonObjectHelper<'a>, ParseError>{
        self.0.as_object()
            .ok_or(ParseError::ExpectedType("object"))
            .map(|o| JsonObjectHelper(o))
    }
}

/// JSON object helper for response parsing.
#[derive(Debug)]
pub struct JsonObjectHelper<'a>(pub &'a json::Object);

impl<'a> JsonObjectHelper<'a> {
    /// Gets a field as a string or returns `None`.
    pub fn get_string_option(&self, key: &'static str) -> Option<&'a str> {
        self.0.get(key).and_then(Json::as_string)
    }

    /// Gets a field as a string or fails with `ParseError::ExpectedFieldType`.
    pub fn get_string(&self, key: &'static str) -> Result<&'a str, ParseError> {
        self.get_string_option(key).ok_or(ParseError::ExpectedFieldType(key, "string"))
    }

    /// Gets a field as an i64 or returns `None`.
    pub fn get_i64_option(&self, key: &'static str) -> Option<i64> {
        self.0.get(key).and_then(Json::as_i64)
    }

    /// Gets a field as an i64 or fails with `ParseError::ExpectedFieldType`.
    pub fn get_i64(&self, key: &'static str) -> Result<i64, ParseError> {
        self.get_i64_option(key).ok_or(ParseError::ExpectedFieldType(key, "i64"))
    }
}
