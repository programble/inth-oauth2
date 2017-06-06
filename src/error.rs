//! Errors.

use std::error::Error;
use std::fmt;

use serde_json::Value;

use client::response::{FromResponse, ParseError};

/// OAuth 2.0 error codes.
///
/// See [RFC 6749, section 5.2](http://tools.ietf.org/html/rfc6749#section-5.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OAuth2ErrorCode {
    /// The request is missing a required parameter, includes an unsupported parameter value (other
    /// than grant type), repeats a parameter, includes multiple credentials, utilizes more than
    /// one mechanism for authenticating the client, or is otherwise malformed.
    InvalidRequest,

    /// Client authentication failed (e.g., unknown client, no client authentication included, or
    /// unsupported authentication method).
    InvalidClient,

    /// The provided authorization grant (e.g., authorization code, resource owner credentials) or
    /// refresh token is invalid, expired, revoked, does not match the redirection URI used in the
    /// authorization request, or was issued to another client.
    InvalidGrant,

    /// The authenticated client is not authorized to use this authorization grant type.
    UnauthorizedClient,

    /// The authorization grant type is not supported by the authorization server.
    UnsupportedGrantType,

    /// The requested scope is invalid, unknown, malformed, or exceeds the scope granted by the
    /// resource owner.
    InvalidScope,

    /// An unrecognized error code, not defined in RFC 6749.
    Unrecognized(String),
}

impl<'a> From<&'a str> for OAuth2ErrorCode {
    fn from(s: &str) -> OAuth2ErrorCode {
        match s {
            "invalid_request" => OAuth2ErrorCode::InvalidRequest,
            "invalid_client" => OAuth2ErrorCode::InvalidClient,
            "invalid_grant" => OAuth2ErrorCode::InvalidGrant,
            "unauthorized_client" => OAuth2ErrorCode::UnauthorizedClient,
            "unsupported_grant_type" => OAuth2ErrorCode::UnsupportedGrantType,
            "invalid_scope" => OAuth2ErrorCode::InvalidScope,
            s => OAuth2ErrorCode::Unrecognized(s.to_owned()),
        }
    }
}

/// OAuth 2.0 error.
///
/// See [RFC 6749, section 5.2](http://tools.ietf.org/html/rfc6749#section-5.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OAuth2Error {
    /// Error code.
    pub code: OAuth2ErrorCode,

    /// Human-readable text providing additional information about the error.
    pub description: Option<String>,

    /// A URI identifying a human-readable web page with information about the error.
    pub uri: Option<String>,
}

impl fmt::Display for OAuth2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.code)?;
        if let Some(ref description) = self.description {
            write!(f, ": {}", description)?;
        }
        if let Some(ref uri) = self.uri {
            write!(f, " ({})", uri)?;
        }
        Ok(())
    }
}

impl Error for OAuth2Error {
    fn description(&self) -> &str { "OAuth 2.0 API error" }
}

impl FromResponse for OAuth2Error {
    fn from_response(json: &Value) -> Result<Self, ParseError> {
        let obj = json.as_object().ok_or(ParseError::ExpectedType("object"))?;

        let code = obj.get("error")
            .and_then(Value::as_str)
            .ok_or(ParseError::ExpectedFieldType("error", "string"))?;
        let description = obj.get("error_description").and_then(Value::as_str);
        let uri = obj.get("error_uri").and_then(Value::as_str);

        Ok(OAuth2Error {
            code: code.into(),
            description: description.map(Into::into),
            uri: uri.map(Into::into),
        })
    }
}

#[cfg(test)]
mod tests {
    use client::response::{FromResponse, ParseError};
    use super::{OAuth2Error, OAuth2ErrorCode};

    #[test]
    fn from_response_empty() {
        let json = "{}".parse().unwrap();
        assert_eq!(
            ParseError::ExpectedFieldType("error", "string"),
            OAuth2Error::from_response(&json).unwrap_err()
        );
    }

    #[test]
    fn from_response() {
        let json = r#"{"error":"invalid_request"}"#.parse().unwrap();
        assert_eq!(
            OAuth2Error {
                code: OAuth2ErrorCode::InvalidRequest,
                description: None,
                uri: None,
            },
            OAuth2Error::from_response(&json).unwrap()
        );
    }

    #[test]
    fn from_response_with_description() {
        let json = r#"{"error":"invalid_request","error_description":"foo"}"#
            .parse()
            .unwrap();
        assert_eq!(
            OAuth2Error {
                code: OAuth2ErrorCode::InvalidRequest,
                description: Some(String::from("foo")),
                uri: None,
            },
            OAuth2Error::from_response(&json).unwrap()
        );
    }

    #[test]
    fn from_response_with_uri() {
        let json = r#"{"error":"invalid_request","error_uri":"http://example.com"}"#
            .parse()
            .unwrap();
        assert_eq!(
            OAuth2Error {
                code: OAuth2ErrorCode::InvalidRequest,
                description: None,
                uri: Some(String::from("http://example.com")),
            },
            OAuth2Error::from_response(&json).unwrap()
        );
    }
}
