use serde::Deserialize;

/// Telegram user or bot account.
#[derive(Debug, Deserialize)]
pub struct User {
    /// Unique Telegram user identifier.
    pub id: i64,

    /// Whether this account is a bot.
    pub is_bot: bool,

    /// User's first name.
    pub first_name: String,
}
