use super::Method;
use crate::types::{Message, ReplyMarkup};
use serde::Serialize;

/// Edits only the reply markup of an existing message.
#[derive(Debug, Serialize)]
pub struct EditMessageReplyMarkup {
    /// Unique identifier of the target chat.
    pub chat_id: i64,

    /// Identifier of the message to edit.
    pub message_id: u32,

    /// New reply markup, or `None` to remove existing reply markup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl Method for EditMessageReplyMarkup {
    type Response = Message;

    const NAME: &str = "editMessageReplyMarkup";
}
