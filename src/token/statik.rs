use rustc_serialize::json::Json;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::impls::UnitVisitor;

use super::Lifetime;
use client::response::{FromResponse, ParseError, JsonHelper};

/// A static, non-expiring token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, RustcEncodable, RustcDecodable)]
pub struct Static;

impl Lifetime for Static {
    fn expired(&self) -> bool { false }
}

impl FromResponse for Static {
    fn from_response(json: &Json) -> Result<Self, ParseError> {
        let obj = try!(JsonHelper(json).as_object());
        if obj.0.contains_key("expires_in") {
            return Err(ParseError::UnexpectedField("expires_in"));
        }
        Ok(Static)
    }
}

impl Serialize for Static {
    fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_unit_struct("Static")
    }
}

impl Deserialize for Static {
    fn deserialize<D: Deserializer>(deserializer: &mut D) -> Result<Self, D::Error> {
        deserializer.deserialize_unit_struct("Static", UnitVisitor)
            .and(Ok(Static))
    }
}

#[cfg(test)]
mod tests {
    use rustc_serialize::json::Json;
    use serde_json;

    use client::response::{FromResponse, ParseError};
    use super::Static;

    #[test]
    fn from_response() {
        let json = Json::from_str("{}").unwrap();
        assert_eq!(Static, Static::from_response(&json).unwrap());
    }

    #[test]
    fn from_response_with_expires_in() {
        let json = Json::from_str(r#"{"expires_in":3600}"#).unwrap();
        assert_eq!(
            ParseError::UnexpectedField("expires_in"),
            Static::from_response(&json).unwrap_err()
        );
    }

    #[test]
    fn serialize_deserialize() {
        let original = Static;
        let serialized = serde_json::to_value(&original);
        let deserialized = serde_json::from_value(serialized).unwrap();
        assert_eq!(original, deserialized);
    }
}
