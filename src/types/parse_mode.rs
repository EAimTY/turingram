use serde::Serialize;

/// Telegram parse mode used for message text.
#[derive(Debug, Serialize)]
pub enum ParseMode {
    /// Telegram MarkdownV2 parse mode.
    #[serde(rename = "MarkdownV2")]
    Markdown,
    /// Telegram HTML parse mode.
    #[serde(rename = "HTML")]
    Html,
}
