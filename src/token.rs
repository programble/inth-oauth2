use chrono::{DateTime, UTC};

/// OAuth 2.0 access token.
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub expires: Option<DateTime<UTC>>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

impl Token {
    /// Returns true if token is expired.
    pub fn expired(&self) -> bool {
        self.expires.map_or(false, |dt| dt < UTC::now())
    }
}
