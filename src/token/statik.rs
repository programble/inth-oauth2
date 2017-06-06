use serde_json::Value;

use client::response::{FromResponse, ParseError};
use token::Lifetime;

/// A static, non-expiring token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Static;

impl Lifetime for Static {
    fn expired(&self) -> bool { false }
}

impl FromResponse for Static {
    fn from_response(json: &Value) -> Result<Self, ParseError> {
        let obj = json.as_object().ok_or(ParseError::ExpectedType("object"))?;
        if obj.contains_key("expires_in") {
            return Err(ParseError::UnexpectedField("expires_in"));
        }
        Ok(Static)
    }
}

#[cfg(test)]
mod tests {
    use client::response::{FromResponse, ParseError};
    use super::Static;

    #[test]
    fn from_response() {
        let json = "{}".parse().unwrap();
        assert_eq!(Static, Static::from_response(&json).unwrap());
    }

    #[test]
    fn from_response_with_expires_in() {
        let json = r#"{"expires_in":3600}"#.parse().unwrap();
        assert_eq!(
            ParseError::UnexpectedField("expires_in"),
            Static::from_response(&json).unwrap_err()
        );
    }
}
