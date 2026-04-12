use super::MessageEntityType;
use serde::{Deserialize, Serialize};

/// Formatting or semantic entity inside message text.
///
/// The struct currently stores the common Bot API entity fields. Extra fields
/// used by some entity kinds, such as URLs for text links, are not represented.
///
/// Incoming entities inside [`crate::types::MessageKind::Text`] are normalized
/// to UTF-8 byte offsets. Outgoing entities are serialized exactly as provided.
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageEntity {
    /// Entity kind.
    #[serde(rename = "type")]
    pub entity_type: MessageEntityType,

    /// Entity start offset.
    pub offset: u32,

    /// Entity length.
    pub length: u32,
}
