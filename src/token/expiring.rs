use chrono::{DateTime, UTC, Duration};
use serde_json::Value;

use super::Lifetime;
use client::response::{FromResponse, ParseError};

/// An expiring token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expiring {
    expires: DateTime<UTC>,
}

impl Expiring {
    /// Returns the expiry time of the access token.
    pub fn expires(&self) -> &DateTime<UTC> { &self.expires }
}

impl Lifetime for Expiring {
    fn expired(&self) -> bool { self.expires < UTC::now() }
}

impl FromResponse for Expiring {
    fn from_response(json: &Value) -> Result<Self, ParseError> {
        let obj = json.as_object().ok_or(ParseError::ExpectedType("object"))?;

        if obj.contains_key("refresh_token") {
            return Err(ParseError::UnexpectedField("refresh_token"));
        }

        let expires_in = obj.get("expires_in")
            .and_then(Value::as_i64)
            .ok_or(ParseError::ExpectedFieldType("expires_in", "i64"))?;

        Ok(Expiring {
            expires: UTC::now() + Duration::seconds(expires_in),
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{UTC, Duration};

    use client::response::FromResponse;
    use super::Expiring;

    #[test]
    fn from_response() {
        let json = r#"{"expires_in":3600}"#.parse().unwrap();
        let expiring = Expiring::from_response(&json).unwrap();
        assert!(expiring.expires > UTC::now());
        assert!(expiring.expires <= UTC::now() + Duration::seconds(3600));
    }
}
