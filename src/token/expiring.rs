use chrono::{DateTime, UTC, Duration, TimeZone};
use rustc_serialize::json::Json;
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};

use super::Lifetime;
use client::response::{FromResponse, ParseError, JsonHelper};

/// An expiring token.
#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(RustcEncodable, RustcDecodable)]
struct Serializable {
    refresh_token: String,
    expires: i64,
}

impl<'a> From<&'a Expiring> for Serializable {
    fn from(expiring: &Expiring) -> Self {
        Serializable {
            refresh_token: expiring.refresh_token.clone(),
            expires: expiring.expires.timestamp(),
        }
    }
}

impl Into<Expiring> for Serializable {
    fn into(self) -> Expiring {
        Expiring {
            refresh_token: self.refresh_token,
            expires: UTC.timestamp(self.expires, 0),
        }
    }
}

impl Encodable for Expiring {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        Serializable::from(self).encode(s)
    }
}

impl Decodable for Expiring {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        Serializable::decode(d).map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{UTC, Duration, Timelike};
    use rustc_serialize::json::{self, Json};

    use client::response::FromResponse;
    use super::Expiring;

    #[test]
    fn from_response() {
        let json = Json::from_str(r#"{"refresh_token":"aaaaaaaa","expires_in":3600}"#).unwrap();
        let expiring = Expiring::from_response(&json).unwrap();
        assert_eq!("aaaaaaaa", expiring.refresh_token);
        assert!(expiring.expires > UTC::now());
        assert!(expiring.expires <= UTC::now() + Duration::seconds(3600));
    }

    #[test]
    fn from_response_inherit() {
        let json = Json::from_str(r#"{"expires_in":3600}"#).unwrap();
        let prev = Expiring {
            refresh_token: String::from("aaaaaaaa"),
            expires: UTC::now(),
        };
        let expiring = Expiring::from_response_inherit(&json, &prev).unwrap();
        assert_eq!("aaaaaaaa", expiring.refresh_token);
        assert!(expiring.expires > UTC::now());
        assert!(expiring.expires <= UTC::now() + Duration::seconds(3600));
    }

    #[test]
    fn encode_decode() {
        let expiring = Expiring {
            refresh_token: String::from("foo"),
            expires: UTC::now().with_nanosecond(0).unwrap(),
        };
        let json = json::encode(&expiring).unwrap();
        let decoded = json::decode(&json).unwrap();
        assert_eq!(expiring, decoded);
    }
}
