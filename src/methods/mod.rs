//! Typed payloads for Telegram Bot API methods supported by this crate.
//!
//! Each request type implements [`Method`], which binds the serializable payload
//! to a Telegram endpoint name and response type.

use serde::{Deserialize, Serialize};

mod answer_callback_query;
mod edit_message_reply_markup;
mod edit_message_text;
mod leave_chat;
mod send_dice;
mod send_message;

pub use self::{
    answer_callback_query::AnswerCallbackQuery, edit_message_reply_markup::EditMessageReplyMarkup,
    edit_message_text::EditMessageText, leave_chat::LeaveChat, send_dice::SendDice,
    send_message::SendMessage,
};

/// Telegram Bot API method description.
///
/// Implementors are serializable request payloads. The associated response type
/// is used by [`crate::Client::execute`] when decoding Telegram's `result`
/// field.
pub trait Method: Serialize {
    /// Successful response type returned by Telegram for this method.
    type Response: for<'de> Deserialize<'de>;

    /// Telegram Bot API method name, for example `sendMessage`.
    const NAME: &str;
}
