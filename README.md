# turingram

[![Version](https://img.shields.io/crates/v/turingram.svg?style=flat)](https://crates.io/crates/turingram)
[![Documentation](https://img.shields.io/badge/docs-release-brightgreen.svg?style=flat)](https://docs.rs/turingram)
[![License](https://img.shields.io/crates/l/turingram.svg?style=flat)](https://github.com/EAimTY/turingram/blob/master/LICENSE)

`turingram` is a small Telegram Bot API framework for Rust. It provides typed request objects for a focused set of Bot API methods, typed update and response models, and a pluggable HTTP execution layer so the same bot logic can run on different runtimes.

The crate currently targets a compact, explicit subset of the Telegram Bot API. Unsupported update or message payloads are preserved as `serde_json::Value` through `UpdateKind::Other` and `MessageKind::Other`, so bots can ignore unknown payloads without failing deserialization.

## Highlights

- Typed method structs for sending and editing messages, answering callback queries, sending dice, and leaving chats.
- A generic `Client<E>` that works with any executor implementing `client::ClientExecutor`.
- An optional Cloudflare Workers executor behind the `worker-0_8` feature.
- Incoming text message entity offsets are normalized from Telegram's UTF-16 code-unit offsets into Rust UTF-8 byte offsets during deserialization.
- Unknown update and message kinds are retained as raw JSON instead of being rejected.

## Bot API Coverage

Telegram Bot API support is not complete yet. The crate currently focuses on a small set of common message, callback query, and chat actions, with room to add more methods and payload types over time.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
turingram = "0.0.0"
```

Enable the Cloudflare Workers executor when building a Worker:

```toml
[dependencies]
turingram = { version = "0.0.0", features = ["worker-0_8"] }
```

## Sending Requests

Create a `Client` with an executor and the bot token, then pass method structs to `Client::execute`.

```rust,ignore
use turingram::{client::worker_0_8::Executor, methods::SendMessage, Client};

async fn send_hello(
    token: String,
) -> Result<(), turingram::client::Error<turingram::client::worker_0_8::Error>> {
    let mut client = Client::new(Executor::new(), token);
    let _message = client
        .execute(SendMessage {
            chat_id: 123456789,
            text: "Hello from turingram".to_owned(),
            parse_mode: None,
            entities: None,
            reply_parameters: None,
            reply_markup: None,
        })
        .await?;

    Ok(())
}
```

The `worker-0_8` feature provides the executor used above. Other runtimes can be integrated by implementing `ClientExecutor` in application code or an adapter crate.

## Handling Updates

Deserialize incoming webhook or polling payloads into `Update`, then match on the update kind.

```rust,no_run
use turingram::types::{MessageKind, UpdateKind};
use turingram::Update;

fn handle_update(json: &str) -> serde_json::Result<()> {
    let update: Update = serde_json::from_str(json)?;

    match update.kind {
        UpdateKind::Message(message) => match message.kind {
            MessageKind::Text { text, entities } => {
                let _ = (text, entities);
            }
            MessageKind::Other(raw_message_kind) => {
                let _ = raw_message_kind;
            }
        },
        UpdateKind::CallbackQuery(callback_query) => {
            let _ = callback_query;
        }
        UpdateKind::Other(raw_update_kind) => {
            let _ = raw_update_kind;
        }
    }

    Ok(())
}
```

For incoming text messages, `MessageEntity::offset` and `MessageEntity::length` are converted to UTF-8 byte indexes. This makes them usable with Rust string slicing when they are valid character boundaries. When you manually construct outgoing message entities for `SendMessage` or `EditMessageText`, the values are serialized as-is, so provide the offsets expected by Telegram.

## Cloudflare Workers

With the `worker-0_8` feature enabled, `client::worker_0_8::Executor` implements `ClientExecutor` using `worker` 0.8.

```rust,ignore
use turingram::{client::worker_0_8::Executor, Client};

let mut client = Client::new(Executor::new(), "123456:bot-token");
```

The module name includes the dependency major/minor version so future executor implementations can coexist without forcing a single Worker runtime version.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
