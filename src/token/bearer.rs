use hyper::header;
use rustc_serialize::json::Json;

use super::{Token, Lifetime};
use client::response::{FromResponse, ParseError, JsonHelper};

/// The bearer token type.
///
/// See [RFC 6750](http://tools.ietf.org/html/rfc6750).
#[derive(Debug, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Bearer<L: Lifetime> {
    access_token: String,
    scope: Option<String>,
    lifetime: L,
}

impl<L: Lifetime> Token<L> for Bearer<L> {
    fn access_token(&self) -> &str { &self.access_token }
    fn scope(&self) -> Option<&str> { self.scope.as_ref().map(|s| &s[..]) }
    fn lifetime(&self) -> &L { &self.lifetime }
}

impl<'a, L: Lifetime> Into<header::Authorization<header::Bearer>> for &'a Bearer<L> {
    fn into(self) -> header::Authorization<header::Bearer> {
        header::Authorization(header::Bearer { token: self.access_token.clone() })
    }
}

impl<L: Lifetime> Bearer<L> {
    fn from_response_and_lifetime(json: &Json, lifetime: L) -> Result<Self, ParseError> {
        let obj = try!(JsonHelper(json).as_object());

        let token_type = try!(obj.get_string("token_type"));
        if token_type != "Bearer" && token_type != "bearer" {
            return Err(ParseError::ExpectedFieldValue("token_type", "Bearer"));
        }

        let access_token = try!(obj.get_string("access_token"));
        let scope = obj.get_string_option("scope");

        Ok(Bearer {
            access_token: access_token.into(),
            scope: scope.map(Into::into),
            lifetime: lifetime,
        })
    }
}

impl<L: Lifetime> FromResponse for Bearer<L> {
    fn from_response(json: &Json) -> Result<Self, ParseError> {
        let lifetime = try!(FromResponse::from_response(json));
        Bearer::from_response_and_lifetime(json, lifetime)
    }

    fn from_response_inherit(json: &Json, prev: &Self) -> Result<Self, ParseError> {
        let lifetime = try!(FromResponse::from_response_inherit(json, &prev.lifetime));
        Bearer::from_response_and_lifetime(json, lifetime)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{UTC, Duration};
    use rustc_serialize::json::Json;

    use client::response::{FromResponse, ParseError};
    use token::{Static, Expiring};
    use super::Bearer;

    #[test]
    fn from_response_with_invalid_token_type() {
        let json = Json::from_str(r#"{"token_type":"MAC","access_token":"aaaaaaaa"}"#).unwrap();
        assert_eq!(
            ParseError::ExpectedFieldValue("token_type", "Bearer"),
            Bearer::<Static>::from_response(&json).unwrap_err()
        );
    }

    #[test]
    fn from_response_capital_b() {
        let json = Json::from_str(r#"{"token_type":"Bearer","access_token":"aaaaaaaa"}"#).unwrap();
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
        let json = Json::from_str(r#"{"token_type":"bearer","access_token":"aaaaaaaa"}"#).unwrap();
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
        let json = Json::from_str(
            r#"{"token_type":"Bearer","access_token":"aaaaaaaa","scope":"foo"}"#
        ).unwrap();
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
    fn from_response_expiring() {
        let json = Json::from_str(r#"
            {
                "token_type":"Bearer",
                "access_token":"aaaaaaaa",
                "expires_in":3600,
                "refresh_token":"bbbbbbbb"
            }
        "#).unwrap();
        let bearer = Bearer::<Expiring>::from_response(&json).unwrap();
        assert_eq!("aaaaaaaa", bearer.access_token);
        assert_eq!(None, bearer.scope);
        let expiring = bearer.lifetime;
        assert_eq!("bbbbbbbb", expiring.refresh_token());
        assert!(expiring.expires() > &UTC::now());
        assert!(expiring.expires() <= &(UTC::now() + Duration::seconds(3600)));
    }

    #[test]
    fn from_response_inherit_expiring() {
        let json = Json::from_str(r#"
            {
                "token_type":"Bearer",
                "access_token":"aaaaaaaa",
                "expires_in":3600,
                "refresh_token":"bbbbbbbb"
            }
        "#).unwrap();
        let prev = Bearer::<Expiring>::from_response(&json).unwrap();

        let json = Json::from_str(r#"
            {
                "token_type":"Bearer",
                "access_token":"cccccccc",
                "expires_in":3600
            }
        "#).unwrap();
        let bearer = Bearer::<Expiring>::from_response_inherit(&json, &prev).unwrap();
        assert_eq!("cccccccc", bearer.access_token);
        assert_eq!(None, bearer.scope);
        let expiring = bearer.lifetime;
        assert_eq!("bbbbbbbb", expiring.refresh_token());
        assert!(expiring.expires() > &UTC::now());
        assert!(expiring.expires() <= &(UTC::now() + Duration::seconds(3600)));
    }
}
