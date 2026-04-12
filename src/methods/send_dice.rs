use super::Method;
use crate::types::{DiceType, Message, ReplyParameters};
use serde::Serialize;

/// Sends an animated dice-style message.
#[derive(Debug, Serialize)]
pub struct SendDice {
    /// Unique identifier of the target chat.
    pub chat_id: i64,

    /// Dice animation to send.
    pub emoji: DiceType,

    /// Optional message to reply to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
}

impl Method for SendDice {
    type Response = Message;

    const NAME: &str = "sendDice";
}
