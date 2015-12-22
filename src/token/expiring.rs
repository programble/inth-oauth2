use chrono::{DateTime, UTC};

use super::Lifetime;

/// An expiring token.
#[derive(Debug)]
pub struct Expiring {
    refresh_token: String,
    expires: DateTime<UTC>,
}

impl Expiring {
    /// Returns the refresh token.
    ///
    /// See [RFC 6749, section 1.5](http://tools.ietf.org/html/rfc6749#section-1.5).
    pub fn refresh_token(&self) -> &str { &self.refresh_token }

    /// Returns the expiry time of the access token.
    pub fn expires(&self) -> &DateTime<UTC> { &self.expires }
}

impl Lifetime for Expiring {
    fn expired(&self) -> bool { self.expires < UTC::now() }
}
