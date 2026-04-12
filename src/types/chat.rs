use super::ChatType;
use serde::Deserialize;

/// Telegram chat referenced by a message or update.
#[derive(Debug, Deserialize)]
pub struct Chat {
    /// Unique Telegram chat identifier.
    pub id: i64,

    /// Kind of chat.
    #[serde(rename = "type")]
    pub chat_type: ChatType,
}
