use chrono::{DateTime, UTC, Duration};
use rustc_serialize::json::Json;

use super::Lifetime;
use client::response::{FromResponse, ParseError, JsonHelper};

/// An expiring token.
#[derive(Debug, PartialEq, Eq)]
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

impl FromResponse for Expiring {
    fn from_response(json: &Json) -> Result<Self, ParseError> {
        let obj = try!(JsonHelper(json).as_object());

        let refresh_token = try!(obj.get_string("refresh_token"));
        let expires_in = try!(obj.get_i64("expires_in"));

        Ok(Expiring {
            refresh_token: refresh_token.into(),
            expires: UTC::now() + Duration::seconds(expires_in),
        })
    }

    fn from_response_inherit(json: &Json, prev: &Self) -> Result<Self, ParseError> {
        let obj = try!(JsonHelper(json).as_object());

        let refresh_token = try! {
            obj.get_string("refresh_token")
                .or(Ok(&prev.refresh_token))
        };
        let expires_in = try!(obj.get_i64("expires_in"));

        Ok(Expiring {
            refresh_token: refresh_token.into(),
            expires: UTC::now() + Duration::seconds(expires_in),
        })
    }
}
