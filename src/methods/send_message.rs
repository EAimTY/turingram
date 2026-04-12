use super::Method;
use crate::types::{Message, MessageEntity, ParseMode, ReplyMarkup, ReplyParameters};
use serde::Serialize;

/// Sends a text message.
#[derive(Debug, Serialize)]
pub struct SendMessage {
    /// Unique identifier of the target chat.
    pub chat_id: i64,

    /// Message text.
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

    /// Optional message to reply to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    /// Optional inline keyboard or other supported reply markup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl Method for SendMessage {
    type Response = Message;

    const NAME: &str = "sendMessage";
}
