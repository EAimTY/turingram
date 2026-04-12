use super::{Message, User};
use serde::Deserialize;

/// Callback query sent when a user presses an inline keyboard button.
#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query.
    pub id: String,

    /// User who pressed the button.
    pub from: User,

    /// Message that contained the pressed button, if Telegram included it.
    pub message: Option<Message>,

    /// Callback data attached to the pressed button.
    pub data: Option<String>,
}
