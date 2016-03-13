use chrono::{DateTime, UTC, Duration, TimeZone};
use rustc_serialize::json::Json;
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};

use super::Lifetime;
use client::response::{FromResponse, ParseError, JsonHelper};

/// An expiring token which can be refreshed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refresh {
    refresh_token: String,
    expires: DateTime<UTC>,
}

impl Refresh {
    /// Returns the refresh token.
    ///
    /// See [RFC 6749, section 1.5](http://tools.ietf.org/html/rfc6749#section-1.5).
    pub fn refresh_token(&self) -> &str { &self.refresh_token }

    /// Returns the expiry time of the access token.
    pub fn expires(&self) -> &DateTime<UTC> { &self.expires }
}

impl Lifetime for Refresh {
    fn expired(&self) -> bool { self.expires < UTC::now() }
}

impl FromResponse for Refresh {
    fn from_response(json: &Json) -> Result<Self, ParseError> {
        let obj = try!(JsonHelper(json).as_object());

        let refresh_token = try!(obj.get_string("refresh_token"));
        let expires_in = try!(obj.get_i64("expires_in"));

        Ok(Refresh {
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

        Ok(Refresh {
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

impl<'a> From<&'a Refresh> for Serializable {
    fn from(refresh: &Refresh) -> Self {
        Serializable {
            refresh_token: refresh.refresh_token.clone(),
            expires: refresh.expires.timestamp(),
        }
    }
}

impl Into<Refresh> for Serializable {
    fn into(self) -> Refresh {
        Refresh {
            refresh_token: self.refresh_token,
            expires: UTC.timestamp(self.expires, 0),
        }
    }
}

impl Encodable for Refresh {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        Serializable::from(self).encode(s)
    }
}

impl Decodable for Refresh {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        Serializable::decode(d).map(Into::into)
    }
}

#[cfg(feature = "serde")]
mod serde {
    use chrono::{UTC, TimeZone};
    use serde::{Serialize, Serializer, Deserialize, Deserializer};
    use serde::{ser, de};

    use super::Refresh;

    impl Serialize for Refresh {
        fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
            serializer.serialize_struct("Refresh", SerVisitor(self, 0))
        }
    }

    struct SerVisitor<'a>(&'a Refresh, u8);
    impl<'a> ser::MapVisitor for SerVisitor<'a> {
        fn visit<S: Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
            self.1 += 1;
            match self.1 {
                1 => serializer.serialize_struct_elt("refresh_token", &self.0.refresh_token).map(Some),
                2 => serializer.serialize_struct_elt("expires", &self.0.expires.timestamp()).map(Some),
                _ => Ok(None),
            }
        }

        fn len(&self) -> Option<usize> { Some(2) }
    }

    impl Deserialize for Refresh {
        fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
            static FIELDS: &'static [&'static str] = &["refresh_token", "expires"];
            deserializer.deserialize_struct("Refresh", FIELDS, DeVisitor)
        }
    }

    struct DeVisitor;
    impl de::Visitor for DeVisitor {
        type Value = Refresh;

        fn visit_map<V: de::MapVisitor>(&mut self, mut visitor: V) -> Result<Refresh, V::Error> {
            let mut refresh_token = None;
            let mut expires = None;

            loop {
                match try!(visitor.visit_key()) {
                    Some(Field::RefreshToken) => refresh_token = Some(try!(visitor.visit_value())),
                    Some(Field::Expires) => expires = Some(try!(visitor.visit_value())),
                    None => break,
                }
            }

            let refresh_token = match refresh_token {
                Some(s) => s,
                None => return visitor.missing_field("refresh_token"),
            };
            let expires = match expires {
                Some(i) => UTC.timestamp(i, 0),
                None => return visitor.missing_field("expires"),
            };

            try!(visitor.end());

            Ok(Refresh {
                refresh_token: refresh_token,
                expires: expires,
            })
        }
    }

    enum Field {
        RefreshToken,
        Expires,
    }

    impl Deserialize for Field {
        fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
            deserializer.deserialize(FieldVisitor)
        }
    }

    struct FieldVisitor;
    impl de::Visitor for FieldVisitor {
        type Value = Field;

        fn visit_str<E: de::Error>(&mut self, value: &str) -> Result<Field, E> {
            match value {
                "refresh_token" => Ok(Field::RefreshToken),
                "expires" => Ok(Field::Expires),
                _ => Err(de::Error::custom("expected refresh_token or expires")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{UTC, Duration, Timelike};
    use rustc_serialize::json::{self, Json};

    use client::response::FromResponse;
    use super::Refresh;

    #[test]
    fn from_response() {
        let json = Json::from_str(r#"{"refresh_token":"aaaaaaaa","expires_in":3600}"#).unwrap();
        let refresh = Refresh::from_response(&json).unwrap();
        assert_eq!("aaaaaaaa", refresh.refresh_token);
        assert!(refresh.expires > UTC::now());
        assert!(refresh.expires <= UTC::now() + Duration::seconds(3600));
    }

    #[test]
    fn from_response_inherit() {
        let json = Json::from_str(r#"{"expires_in":3600}"#).unwrap();
        let prev = Refresh {
            refresh_token: String::from("aaaaaaaa"),
            expires: UTC::now(),
        };
        let refresh = Refresh::from_response_inherit(&json, &prev).unwrap();
        assert_eq!("aaaaaaaa", refresh.refresh_token);
        assert!(refresh.expires > UTC::now());
        assert!(refresh.expires <= UTC::now() + Duration::seconds(3600));
    }

    #[test]
    fn encode_decode() {
        let refresh = Refresh {
            refresh_token: String::from("foo"),
            expires: UTC::now().with_nanosecond(0).unwrap(),
        };
        let json = json::encode(&refresh).unwrap();
        let decoded = json::decode(&json).unwrap();
        assert_eq!(refresh, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_deserialize() {
        use serde_json;

        let original = Refresh {
            refresh_token: String::from("foo"),
            expires: UTC::now().with_nanosecond(0).unwrap(),
        };
        let serialized = serde_json::to_value(&original);
        let deserialized = serde_json::from_value(serialized).unwrap();
        assert_eq!(original, deserialized);
    }
}
