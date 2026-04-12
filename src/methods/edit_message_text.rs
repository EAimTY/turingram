use super::Method;
use crate::types::{Message, MessageEntity, ParseMode, ReplyMarkup};
use serde::Serialize;

/// Edits the text and optional inline keyboard of an existing message.
#[derive(Debug, Serialize)]
pub struct EditMessageText {
    /// Unique identifier of the target chat.
    pub chat_id: i64,

    /// Identifier of the message to edit.
    pub message_id: u32,

    /// New message text.
    pub text: String,

    /// Optional parse mode used by Telegram when parsing `text`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// Explicit message entities for `text`.
    ///
    /// These values are serialized as-is. When sending entities to Telegram,
    /// provide offsets and lengths in the units expected by the Bot API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    /// Optional replacement inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl Method for EditMessageText {
    type Response = Message;

    const NAME: &str = "editMessageText";
}
