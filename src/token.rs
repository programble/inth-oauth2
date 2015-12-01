use std::ops::Deref;

use chrono::{DateTime, UTC, TimeZone};
use hyper::header;
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};

/// OAuth 2.0 access token and refresh token pair.
#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct TokenPair {
    /// The access token.
    pub access: AccessToken,
    /// The refresh token.
    pub refresh: Option<RefreshToken>,
}

/// OAuth 2.0 access token type.
///
/// See [RFC6749 section 7.1](http://tools.ietf.org/html/rfc6749#section-7.1).
#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub enum AccessTokenType {
    /// The bearer token type.
    ///
    /// See [RFC6750](http://tools.ietf.org/html/rfc6750).
    Bearer,

    /// An unrecognized token type.
    Unrecognized(String),
}

/// OAuth 2.0 access token.
///
/// See [RFC6749 section 5](http://tools.ietf.org/html/rfc6749#section-5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken {
    /// The access token issued by the authorization server.
    pub token: String,

    /// The type of the token issued.
    pub token_type: AccessTokenType,

    /// The expiry time of the access token.
    pub expires: Option<DateTime<UTC>>,

    /// The scope of the access token.
    pub scope: Option<String>,
}

/// OAuth 2.0 refresh token.
///
/// See [RFC6749 section 1.5](http://tools.ietf.org/html/rfc6749#section-1.5).
#[derive(Debug, Clone, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct RefreshToken {
    /// The refresh token issued by the authorization server.
    pub token: String,
}

impl AccessToken {
    /// Returns true if token is expired.
    pub fn expired(&self) -> bool {
        self.expires.map_or(false, |dt| dt < UTC::now())
    }

    /// Creates an Authorization header.
    ///
    /// Returns `None` if `token_type` is not `Bearer`.
    pub fn to_bearer_header(&self) -> Option<header::Authorization<header::Bearer>> {
        if self.token_type == AccessTokenType::Bearer {
            Some(header::Authorization(header::Bearer { token: self.token.clone() }))
        } else {
            None
        }
    }
}

impl Deref for TokenPair {
    type Target = AccessToken;

    fn deref(&self) -> &AccessToken {
        &self.access
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct SerializableAccessToken {
    token: String,
    token_type: AccessTokenType,
    expires: Option<i64>,
    scope: Option<String>,
}

impl SerializableAccessToken {
    fn from_access_token(access: &AccessToken) -> Self {
        SerializableAccessToken {
            token: access.token.clone(),
            token_type: access.token_type.clone(),
            expires: access.expires.as_ref().map(DateTime::timestamp),
            scope: access.scope.clone(),
        }
    }

    fn into_access_token(self) -> AccessToken {
        AccessToken {
            token: self.token,
            token_type: self.token_type,
            expires: self.expires.map(|t| UTC.timestamp(t, 0)),
            scope: self.scope,
        }
    }
}

impl Encodable for AccessToken {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        SerializableAccessToken::from_access_token(self).encode(s)
    }
}

impl Decodable for AccessToken {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        SerializableAccessToken::decode(d)
            .map(SerializableAccessToken::into_access_token)
    }
}
