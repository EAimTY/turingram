use super::{CallbackQuery, Message};
use serde::{
    Deserialize, Deserializer,
    de::{MapAccess, Visitor},
};
use serde_json::Value as JsonValue;
use std::fmt::{Formatter, Result as FmtResult};

/// Supported Telegram update payloads.
#[derive(Debug)]
pub enum UpdateKind {
    /// New incoming message.
    Message(Message),

    /// Callback query from an inline keyboard.
    CallbackQuery(CallbackQuery),

    /// Raw JSON for update kinds not modeled by this crate.
    Other(JsonValue),
}

impl<'de> Deserialize<'de> for UpdateKind {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        struct UpdateKindVisitor;

        impl<'de> Visitor<'de> for UpdateKindVisitor {
            type Value = UpdateKind;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("update")
            }

            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Self::Value, M::Error> {
                let Some(key) = map.next_key::<String>().unwrap() else {
                    return Ok(UpdateKind::Other(JsonValue::Null));
                };

                let res = match key.as_str() {
                    "message" => Ok(UpdateKind::Message(map.next_value()?)),
                    "callback_query" => Ok(UpdateKind::CallbackQuery(map.next_value()?)),
                    _ => Ok(UpdateKind::Other(map.next_value()?)),
                };

                assert!(map.next_key::<String>().unwrap().is_none());

                res
            }
        }

        de.deserialize_any(UpdateKindVisitor)
    }
}
