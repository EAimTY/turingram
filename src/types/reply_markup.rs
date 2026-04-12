use super::InlineKeyboardButton;
use serde::Serialize;

/// Supported reply markup payloads.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    /// Inline keyboard made of button rows.
    InlineKeyboard {
        /// Rows of inline keyboard buttons.
        inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
    },
}
