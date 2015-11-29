use chrono::{DateTime, UTC, TimeZone};
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};

/// OAuth 2.0 access token.
///
/// See [RFC6749 section 5](http://tools.ietf.org/html/rfc6749#section-5).
#[derive(Debug, Clone)]
pub struct Token {
    /// The access token issued by the authorization server.
    pub access_token: String,

    /// The type of the token issued.
    ///
    /// See [RFC6749 section 7.1](http://tools.ietf.org/html/rfc6749#section-7.1).
    pub token_type: String,

    /// The expiry time of the access token.
    pub expires: Option<DateTime<UTC>>,

    /// The refresh token, which can be used to obtain new access tokens.
    pub refresh_token: Option<String>,

    /// The scope of the access token.
    pub scope: Option<String>,
}

impl Token {
    /// Returns true if token is expired.
    pub fn expired(&self) -> bool {
        self.expires.map_or(false, |dt| dt < UTC::now())
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct SerializableToken {
    access_token: String,
    token_type: String,
    expires: Option<i64>,
    refresh_token: Option<String>,
    scope: Option<String>,
}

impl SerializableToken {
    fn from_token(token: &Token) -> Self {
        SerializableToken {
            access_token: token.access_token.clone(),
            token_type: token.token_type.clone(),
            expires: token.expires.as_ref().map(DateTime::timestamp),
            refresh_token: token.refresh_token.clone(),
            scope: token.scope.clone(),
        }
    }

    fn into_token(self) -> Token {
        Token {
            access_token: self.access_token,
            token_type: self.token_type,
            expires: self.expires.map(|t| UTC.timestamp(t, 0)),
            refresh_token: self.refresh_token,
            scope: self.scope,
        }
    }
}

impl Encodable for Token {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        SerializableToken::from_token(self).encode(s)
    }
}

impl Decodable for Token {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        SerializableToken::decode(d).map(SerializableToken::into_token)
    }
}
