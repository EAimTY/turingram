use super::UpdateKind;
use serde::Deserialize;

/// Telegram update delivered through webhooks or polling.
#[derive(Debug, Deserialize)]
pub struct Update {
    /// Unique update identifier.
    #[serde(rename = "update_id")]
    pub id: u32,

    /// Typed or raw update payload.
    #[serde(flatten)]
    pub kind: UpdateKind,
}
