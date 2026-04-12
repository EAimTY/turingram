use serde::Serialize;

/// Parameters identifying the message being replied to.
#[derive(Debug, Serialize)]
pub struct ReplyParameters {
    /// Identifier of the message that the new message replies to.
    pub message_id: u32,
}
