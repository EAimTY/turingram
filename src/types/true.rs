use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Marker type for Telegram methods whose successful result is the JSON value
/// `true`.
///
/// Serialization always emits `true`; deserialization accepts only `true`.
#[derive(Debug)]
pub struct True;

impl Serialize for True {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_bool(true)
    }
}

impl<'de> Deserialize<'de> for True {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        assert!(bool::deserialize(de)?);
        Ok(Self)
    }
}
