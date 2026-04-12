use super::Method;
use crate::types::True;
use serde::Serialize;
use std::ops::Not;

/// Answers a callback query from an inline keyboard button.
///
/// Telegram clients show a progress indicator after a callback button is
/// pressed. This method stops that indicator and can optionally display a
/// notification or alert to the user.
#[derive(Debug, Serialize)]
pub struct AnswerCallbackQuery {
    /// Unique identifier of the callback query to answer.
    pub callback_query_id: String,

    /// Optional notification text shown to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Whether to show `text` as an alert instead of a transient notification.
    ///
    /// `false` is omitted from the JSON payload.
    #[serde(skip_serializing_if = "Not::not")]
    pub show_alert: bool,
}

impl Method for AnswerCallbackQuery {
    type Response = True;

    const NAME: &str = "answerCallbackQuery";
}
