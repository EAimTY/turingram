use super::MessageEntity;
use serde::Deserialize;
use serde_json::Value as JsonValue;

/// Supported Telegram message payloads.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MessageKind {
    /// Text message payload.
    Text {
        /// Message text.
        text: String,

        /// Message entities contained in `text`.
        ///
        /// Incoming offsets and lengths are normalized to UTF-8 byte indexes by
        /// [`crate::types::Message`] during deserialization.
        #[serde(default)]
        entities: Vec<MessageEntity>,
    },

    /// Raw JSON for message kinds not modeled by this crate.
    Other(JsonValue),
}
