use chrono::{DateTime, Utc, Duration};
use serde_json::Value;

use client::response::{FromResponse, ParseError};
use token::Lifetime;

/// An expiring token which can be refreshed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Refresh {
    refresh_token: String,
    expires: DateTime<Utc>,
}

impl Refresh {
    /// Returns the refresh token.
    ///
    /// See [RFC 6749, section 1.5](http://tools.ietf.org/html/rfc6749#section-1.5).
    pub fn refresh_token(&self) -> &str { &self.refresh_token }

    /// Returns the expiry time of the access token.
    pub fn expires(&self) -> &DateTime<Utc> { &self.expires }
}

impl Lifetime for Refresh {
    fn expired(&self) -> bool { self.expires < Utc::now() }
}

impl FromResponse for Refresh {
    fn from_response(json: &Value) -> Result<Self, ParseError> {
        let obj = json.as_object().ok_or(ParseError::ExpectedType("object"))?;

        let refresh_token = obj.get("refresh_token")
            .and_then(Value::as_str)
            .ok_or(ParseError::ExpectedFieldType("refresh_token", "string"))?;
        let expires_in = obj.get("expires_in")
            .and_then(Value::as_i64)
            .ok_or(ParseError::ExpectedFieldType("expires_in", "i64"))?;

        Ok(Refresh {
            refresh_token: refresh_token.into(),
            expires: Utc::now() + Duration::seconds(expires_in),
        })
    }

    fn from_response_inherit(json: &Value, prev: &Self) -> Result<Self, ParseError> {
        let obj = json.as_object().ok_or(ParseError::ExpectedType("object"))?;

        let refresh_token = obj.get("refresh_token")
            .and_then(Value::as_str)
            .or(Some(&prev.refresh_token))
            .ok_or(ParseError::ExpectedFieldType("refresh_token", "string"))?;

        let expires_in = obj.get("expires_in")
            .and_then(Value::as_i64)
            .ok_or(ParseError::ExpectedFieldType("expires_in", "i64"))?;

        Ok(Refresh {
            refresh_token: refresh_token.into(),
            expires: Utc::now() + Duration::seconds(expires_in),
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, Duration};

    use client::response::FromResponse;
    use super::Refresh;

    #[test]
    fn from_response() {
        let json = r#"{"refresh_token":"aaaaaaaa","expires_in":3600}"#.parse().unwrap();
        let refresh = Refresh::from_response(&json).unwrap();
        assert_eq!("aaaaaaaa", refresh.refresh_token);
        assert!(refresh.expires > Utc::now());
        assert!(refresh.expires <= Utc::now() + Duration::seconds(3600));
    }

    #[test]
    fn from_response_inherit() {
        let json = r#"{"expires_in":3600}"#.parse().unwrap();
        let prev = Refresh {
            refresh_token: String::from("aaaaaaaa"),
            expires: Utc::now(),
        };
        let refresh = Refresh::from_response_inherit(&json, &prev).unwrap();
        assert_eq!("aaaaaaaa", refresh.refresh_token);
        assert!(refresh.expires > Utc::now());
        assert!(refresh.expires <= Utc::now() + Duration::seconds(3600));
    }
}
