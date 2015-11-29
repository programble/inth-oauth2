use chrono::{DateTime, UTC};

/// OAuth 2.0 access token.
///
/// See [RFC6749 section 5](http://tools.ietf.org/html/rfc6749#section-5).
#[derive(Debug)]
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
