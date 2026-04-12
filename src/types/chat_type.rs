use serde::Deserialize;

/// Telegram chat category.
#[derive(Debug, Deserialize)]
pub enum ChatType {
    /// One-on-one chat with a user.
    #[serde(rename = "private")]
    Private,
    /// Basic group chat.
    #[serde(rename = "group")]
    Group,
    /// Supergroup chat.
    #[serde(rename = "supergroup")]
    Supergroup,
    /// Channel chat.
    #[serde(rename = "channel")]
    Channel,
}
