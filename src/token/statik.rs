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

#[cfg(test)]
mod tests {
    use rustc_serialize::json::Json;

    use client::response::{FromResponse, ParseError};
    use super::Static;

    #[test]
    fn from_response() {
        let json = Json::from_str("{}").unwrap();
        assert_eq!(Static, Static::from_response(&json).unwrap());
    }

    #[test]
    fn from_response_with_expires_in() {
        let json = Json::from_str(r#"{"expires_in":3600}"#).unwrap();
        assert_eq!(
            ParseError::UnexpectedField("expires_in"),
            Static::from_response(&json).unwrap_err()
        );
    }
}
