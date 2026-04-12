#![doc = include_str!("../README.md")]

/// Client and executor abstractions used to call the Telegram Bot API.
pub mod client;
/// Typed Telegram Bot API method payloads.
pub mod methods;
/// Telegram Bot API request helpers, responses, and incoming update models.
pub mod types;

pub use crate::{client::Client, methods::Method, types::Update};
