use super::Method;
use crate::types::True;
use serde::Serialize;

/// Makes the bot leave a group, supergroup, or channel.
#[derive(Debug, Serialize)]
pub struct LeaveChat {
    /// Unique identifier of the chat the bot should leave.
    pub chat_id: i64,
}

impl Method for LeaveChat {
    type Response = True;

    const NAME: &str = "leaveChat";
}
