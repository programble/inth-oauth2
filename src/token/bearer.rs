use serde_json::Value;

use client::response::{FromResponse, ParseError};
use token::{Token, Lifetime};

/// The bearer token type.
///
/// See [RFC 6750](http://tools.ietf.org/html/rfc6750).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bearer<L: Lifetime> {
    access_token: String,
    scope: Option<String>,
    lifetime: L,
}

impl<L: Lifetime> Token<L> for Bearer<L> {
    fn access_token(&self) -> &str {
        &self.access_token
    }
    fn scope(&self) -> Option<&str> {
        self.scope.as_ref().map(|s| &s[..])
    }
    fn lifetime(&self) -> &L {
        &self.lifetime
    }
}

impl<L: Lifetime> Bearer<L> {
    fn from_response_and_lifetime(json: &Value, lifetime: L) -> Result<Self, ParseError> {
        let obj = json.as_object().ok_or(ParseError::ExpectedType("object"))?;

        let token_type = obj.get("token_type")
            .and_then(Value::as_str)
            .ok_or(ParseError::ExpectedFieldType("token_type", "string"))?;
        if token_type != "Bearer" && token_type != "bearer" {
            return Err(ParseError::ExpectedFieldValue("token_type", "Bearer"));
        }

        let access_token = obj.get("access_token")
            .and_then(Value::as_str)
            .ok_or(ParseError::ExpectedFieldType("access_token", "string"))?;
        let scope = obj.get("scope").and_then(Value::as_str);

        Ok(Bearer {
            access_token: access_token.into(),
            scope: scope.map(Into::into),
            lifetime: lifetime,
        })
    }
}

impl<L: Lifetime> FromResponse for Bearer<L> {
    fn from_response(json: &Value) -> Result<Self, ParseError> {
        let lifetime = FromResponse::from_response(json)?;
        Bearer::from_response_and_lifetime(json, lifetime)
    }

    fn from_response_inherit(json: &Value, prev: &Self) -> Result<Self, ParseError> {
        let lifetime = FromResponse::from_response_inherit(json, &prev.lifetime)?;
        Bearer::from_response_and_lifetime(json, lifetime)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Utc, Duration};

    use client::response::{FromResponse, ParseError};
    use token::{Static, Refresh};
    use super::Bearer;

    #[test]
    fn from_response_with_invalid_token_type() {
        let json = r#"{"token_type":"MAC","access_token":"aaaaaaaa"}"#.parse().unwrap();
        assert_eq!(
            ParseError::ExpectedFieldValue("token_type", "Bearer"),
            Bearer::<Static>::from_response(&json).unwrap_err()
        );
    }

    #[test]
    fn from_response_capital_b() {
        let json = r#"{"token_type":"Bearer","access_token":"aaaaaaaa"}"#.parse().unwrap();
        assert_eq!(
            Bearer {
                access_token: String::from("aaaaaaaa"),
                scope: None,
                lifetime: Static,
            },
            Bearer::<Static>::from_response(&json).unwrap()
        );
    }

    #[test]
    fn from_response_little_b() {
        let json = r#"{"token_type":"bearer","access_token":"aaaaaaaa"}"#.parse().unwrap();
        assert_eq!(
            Bearer {
                access_token: String::from("aaaaaaaa"),
                scope: None,
                lifetime: Static,
            },
            Bearer::<Static>::from_response(&json).unwrap()
        );
    }

    #[test]
    fn from_response_with_scope() {
        let json = r#"{"token_type":"Bearer","access_token":"aaaaaaaa","scope":"foo"}"#
            .parse()
            .unwrap();
        assert_eq!(
            Bearer {
                access_token: String::from("aaaaaaaa"),
                scope: Some(String::from("foo")),
                lifetime: Static,
            },
            Bearer::<Static>::from_response(&json).unwrap()
        );
    }

    #[test]
    fn from_response_refresh() {
        let json = r#"
            {
                "token_type":"Bearer",
                "access_token":"aaaaaaaa",
                "expires_in":3600,
                "refresh_token":"bbbbbbbb"
            }
        "#.parse().unwrap();
        let bearer = Bearer::<Refresh>::from_response(&json).unwrap();
        assert_eq!("aaaaaaaa", bearer.access_token);
        assert_eq!(None, bearer.scope);
        let refresh = bearer.lifetime;
        assert_eq!("bbbbbbbb", refresh.refresh_token());
        assert!(refresh.expires() > &Utc::now());
        assert!(refresh.expires() <= &(Utc::now() + Duration::seconds(3600)));
    }

    #[test]
    fn from_response_inherit_refresh() {
        let json = r#"
            {
                "token_type":"Bearer",
                "access_token":"aaaaaaaa",
                "expires_in":3600,
                "refresh_token":"bbbbbbbb"
            }
        "#.parse().unwrap();
        let prev = Bearer::<Refresh>::from_response(&json).unwrap();

        let json = r#"
            {
                "token_type":"Bearer",
                "access_token":"cccccccc",
                "expires_in":3600
            }
        "#.parse().unwrap();
        let bearer = Bearer::<Refresh>::from_response_inherit(&json, &prev).unwrap();
        assert_eq!("cccccccc", bearer.access_token);
        assert_eq!(None, bearer.scope);
        let refresh = bearer.lifetime;
        assert_eq!("bbbbbbbb", refresh.refresh_token());
        assert!(refresh.expires() > &Utc::now());
        assert!(refresh.expires() <= &(Utc::now() + Duration::seconds(3600)));
    }
}
