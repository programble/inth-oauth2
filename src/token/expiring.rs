use chrono::{DateTime, UTC, Duration, TimeZone};
use rustc_serialize::json::Json;
use rustc_serialize::{Encodable, Encoder, Decodable, Decoder};

use super::Lifetime;
use client::response::{FromResponse, ParseError, JsonHelper};

/// An expiring token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Expiring {
    expires: DateTime<UTC>,
}

impl Expiring {
    /// Returns the expiry time of the access token.
    pub fn expires(&self) -> &DateTime<UTC> { &self.expires }
}

impl Lifetime for Expiring {
    fn expired(&self) -> bool { self.expires < UTC::now() }
}

impl FromResponse for Expiring {
    fn from_response(json: &Json) -> Result<Self, ParseError> {
        let obj = try!(JsonHelper(json).as_object());

        if obj.0.contains_key("refresh_token") {
            return Err(ParseError::UnexpectedField("refresh_token"));
        }

        let expires_in = try!(obj.get_i64("expires_in"));

        Ok(Expiring {
            expires: UTC::now() + Duration::seconds(expires_in),
        })
    }
}

#[derive(RustcEncodable, RustcDecodable)]
struct Serializable {
    expires: i64,
}

impl<'a> From<&'a Expiring> for Serializable {
    fn from(expiring: &Expiring) -> Self {
        Serializable {
            expires: expiring.expires.timestamp(),
        }
    }
}

impl Into<Expiring> for Serializable {
    fn into(self) -> Expiring {
        Expiring {
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

#[cfg(feature = "serde")]
mod serde {
    use chrono::{UTC, TimeZone};
    use serde::{Serialize, Serializer, Deserialize, Deserializer};
    use serde::{ser, de};

    use super::Expiring;

    impl Serialize for Expiring {
        fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
            serializer.serialize_struct("Expiring", SerVisitor(self, 0))
        }
    }

    struct SerVisitor<'a>(&'a Expiring, u8);
    impl<'a> ser::MapVisitor for SerVisitor<'a> {
        fn visit<S: Serializer>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> {
            self.1 += 1;
            match self.1 {
                1 => serializer.serialize_struct_elt("expires", &self.0.expires.timestamp()).map(Some),
                _ => Ok(None),
            }
        }

        fn len(&self) -> Option<usize> { Some(1) }
    }

    impl Deserialize for Expiring {
        fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
            static FIELDS: &'static [&'static str] = &["expires"];
            deserializer.deserialize_struct("Expiring", FIELDS, DeVisitor)
        }
    }

    struct DeVisitor;
    impl de::Visitor for DeVisitor {
        type Value = Expiring;

        fn visit_map<V: de::MapVisitor>(&mut self, mut visitor: V) -> Result<Expiring, V::Error> {
            let mut expires = None;

            loop {
                match try!(visitor.visit_key()) {
                    Some(Field::Expires) => expires = Some(try!(visitor.visit_value())),
                    None => break,
                }
            }

            let expires = match expires {
                Some(i) => UTC.timestamp(i, 0),
                None => return visitor.missing_field("expires"),
            };

            try!(visitor.end());

            Ok(Expiring {
                expires: expires,
            })
        }
    }

    enum Field {
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
                "expires" => Ok(Field::Expires),
                _ => Err(de::Error::custom("expected expires")),
            }
        }
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
        let json = Json::from_str(r#"{"expires_in":3600}"#).unwrap();
        let expiring = Expiring::from_response(&json).unwrap();
        assert!(expiring.expires > UTC::now());
        assert!(expiring.expires <= UTC::now() + Duration::seconds(3600));
    }

    #[test]
    fn encode_decode() {
        let expiring = Expiring {
            expires: UTC::now().with_nanosecond(0).unwrap(),
        };
        let json = json::encode(&expiring).unwrap();
        let decoded = json::decode(&json).unwrap();
        assert_eq!(expiring, decoded);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serialize_deserialize() {
        use serde_json;

        let original = Expiring {
            expires: UTC::now().with_nanosecond(0).unwrap(),
        };
        let serialized = serde_json::to_value(&original);
        let deserialized = serde_json::from_value(serialized).unwrap();
        assert_eq!(original, deserialized);
    }
}
