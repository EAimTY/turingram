use serde::{Deserialize, Serialize};

/// Type of a [`crate::types::MessageEntity`].
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MessageEntityType {
    /// Mention of a user or public chat.
    Mention,
    /// Hashtag.
    Hashtag,
    /// Cashtag.
    Cashtag,
    /// Bot command.
    BotCommand,
    /// URL.
    Url,
    /// Email address.
    Email,
    /// Phone number.
    PhoneNumber,
    /// Bold text.
    Bold,
    /// Italic text.
    Italic,
    /// Underlined text.
    Underline,
    /// Strikethrough text.
    Strikethrough,
    /// Spoiler text.
    Spoiler,
    /// Block quote.
    Blockquote,
    /// Inline code.
    Code,
    /// Preformatted code block.
    Pre,
    /// Text link.
    TextLink,
    /// Mention of a user without a username.
    TextMention,
    /// Custom emoji.
    CustomEmoji,
}
