//! Telegram Bot API data types used by request payloads and incoming updates.
//!
//! The module contains a deliberately small set of strongly typed structures.
//! Unsupported update and message variants are represented by raw JSON so
//! callers can keep processing updates even when Telegram sends data this crate
//! does not model yet.

mod callback_query;
mod chat;
mod chat_type;
mod dice_type;
mod inline_keyboard_button;
mod inline_keyboard_button_kind;
mod message;
mod message_entity;
mod message_entity_type;
mod message_kind;
mod parse_mode;
mod reply_markup;
mod reply_parameters;
mod r#true;
mod update;
mod update_kind;
mod user;

pub use self::{
    callback_query::CallbackQuery, chat::Chat, chat_type::ChatType, dice_type::DiceType,
    inline_keyboard_button::InlineKeyboardButton,
    inline_keyboard_button_kind::InlineKeyboardButtonKind, message::Message,
    message_entity::MessageEntity, message_entity_type::MessageEntityType,
    message_kind::MessageKind, parse_mode::ParseMode, reply_markup::ReplyMarkup,
    reply_parameters::ReplyParameters, r#true::True, update::Update, update_kind::UpdateKind,
    user::User,
};
