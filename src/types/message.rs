use super::{Chat, MessageEntity, MessageKind, User};
use serde::{Deserialize, Deserializer};
use std::cmp::Reverse;

/// Telegram message.
///
/// The deserializer recognizes text messages and preserves all other message
/// payloads as raw JSON through [`MessageKind::Other`]. For text messages,
/// entity offsets are converted from Telegram UTF-16 code-unit indexes to Rust
/// UTF-8 byte indexes.
#[derive(Debug)]
pub struct Message {
    /// Unique message identifier inside the chat.
    pub id: u32,

    /// Unix timestamp of when Telegram sent the message.
    pub date: i64,

    /// Chat containing the message.
    pub chat: Chat,

    /// Sender of the message, when Telegram includes one.
    pub from: Option<User>,

    /// Typed or raw message payload.
    pub kind: MessageKind,
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Data {
            #[serde(rename = "message_id")]
            id: u32,
            date: i64,
            chat: Chat,
            from: Option<User>,
            #[serde(flatten)]
            kind: MessageKind,
        }

        let mut data = Data::deserialize(de)?;

        #[allow(clippy::single_match)]
        match &mut data.kind {
            MessageKind::Text { text, entities } => {
                update_message_entity_index_to_utf8(text, entities)
            }
            _ => (),
        }

        Ok(Self {
            id: data.id,
            date: data.date,
            chat: data.chat,
            from: data.from,
            kind: data.kind,
        })
    }
}

fn update_message_entity_index_to_utf8(text: &str, entities: &mut [MessageEntity]) {
    for entity in entities.iter_mut() {
        entity.length += entity.offset; // temporary store the end index in the length field
    }

    let mut offsets = entities
        .iter_mut()
        .flat_map(|entity| [&mut entity.offset, &mut entity.length])
        .collect::<Vec<_>>();

    offsets.sort_unstable_by_key(|offset| Reverse(**offset));

    let _ = text
        .chars()
        .chain(['\0']) // this is needed to process offset pointing at the end of the string
        .try_fold((0, 0), |(len_utf8, len_utf16), c| {
            if offsets.is_empty() {
                return None;
            }

            while offsets
                .last()
                .map(|offset| **offset <= len_utf16)
                .unwrap_or(false)
            {
                let offset = offsets.pop().unwrap();
                assert_eq!(*offset, len_utf16, "invalid utf-16 offset");
                *offset = len_utf8;
            }

            Some((
                len_utf8 + c.len_utf8() as u32,
                len_utf16 + c.len_utf16() as u32,
            ))
        });

    for entity in entities.iter_mut() {
        entity.length -= entity.offset; // restore the length field
    }
}
