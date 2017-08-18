use chrono::{DateTime, Duration};
use chrono::offset::Utc;
use serde_json::Value;

use client::response::{FromResponse, ParseError};
use token::Lifetime;

/// An expiring token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expiring {
    expires: DateTime<Utc>,
}

impl Expiring {
    /// Returns the expiry time of the access token.
    pub fn expires(&self) -> &DateTime<Utc> {
        &self.expires
    }
}

impl Lifetime for Expiring {
    fn expired(&self) -> bool {
        self.expires < Utc::now()
    }
}

impl FromResponse for Expiring {
    fn from_response(json: &Value) -> Result<Self, ParseError> {
        let obj = json.as_object().ok_or(ParseError::ExpectedType("object"))?;

        if obj.contains_key("refresh_token") {
            return Err(ParseError::UnexpectedField("refresh_token"));
        }

        let expires_in = obj.get("expires_in").and_then(Value::as_i64).ok_or(
            ParseError::ExpectedFieldType("expires_in", "i64"),
        )?;

        Ok(Expiring {
            expires: Utc::now() + Duration::seconds(expires_in),
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, Duration};

    use client::response::FromResponse;
    use super::Expiring;

    #[test]
    fn from_response() {
        let json = r#"{"expires_in":3600}"#.parse().unwrap();
        let expiring = Expiring::from_response(&json).unwrap();
        assert!(expiring.expires > Utc::now());
        assert!(expiring.expires <= Utc::now() + Duration::seconds(3600));
    }
}
