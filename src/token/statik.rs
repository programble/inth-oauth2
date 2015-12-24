use rustc_serialize::json::Json;

use super::Lifetime;
use client::response::{FromResponse, ParseError, JsonHelper};

/// A static, non-expiring token.
#[derive(Debug, PartialEq, Eq)]
pub struct Static;

impl Lifetime for Static {
    fn expired(&self) -> bool { false }
}

impl FromResponse for Static {
    fn from_response(json: &Json) -> Result<Self, ParseError> {
        let obj = try!(JsonHelper(json).as_object());
        if obj.0.contains_key("expires_in") {
            return Err(ParseError::UnexpectedField("expires_in"));
        }
        Ok(Static)
    }
}
