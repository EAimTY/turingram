use serde::Serialize;

/// Supported inline keyboard button behaviors.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum InlineKeyboardButtonKind {
    /// Opens the given URL when the button is pressed.
    Url {
        /// URL opened by the Telegram client.
        url: String,
    },
    /// Sends callback data back to the bot when the button is pressed.
    CallbackData {
        /// Opaque payload delivered in [`crate::types::CallbackQuery::data`].
        callback_data: String,
    },
}
