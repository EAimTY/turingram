//! Telegram Bot API client primitives.
//!
//! The core client is independent from any HTTP runtime. It builds Telegram Bot
//! API URLs, serializes method payloads, deserializes typed responses, and leaves
//! transport details to a [`crate::client::ClientExecutor`].

use crate::Method;
use http::Uri;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Cloudflare Workers executor implementation for `worker` 0.8.
#[cfg(feature = "worker-0_8")]
pub mod worker_0_8;

/// Telegram Bot API client parameterized by an HTTP executor.
///
/// `Client` owns the bot token and delegates all network I/O to `E`. It is
/// mutable because executors often own reusable connection state, request
/// counters, or platform handles.
#[derive(Debug)]
pub struct Client<E> {
    executor: E,
    token: String,
}

/// Runtime-specific transport used by [`Client`].
///
/// Implementors receive the fully formed Telegram Bot API URI and a serializable
/// method payload. They must send the payload as a JSON `POST` request with an
/// `application/json` content type, then deserialize the JSON response body into
/// `U`.
pub trait ClientExecutor {
    /// Error returned by the underlying HTTP runtime or response decoder.
    type Error;

    /// Sends one JSON request to Telegram and decodes the JSON response body.
    fn request<T, U>(
        &mut self,
        uri: Uri,
        payload: T,
    ) -> impl Future<Output = Result<U, Self::Error>> + Send
    where
        T: Serialize + Send,
        U: for<'a> Deserialize<'a>;
}

/// Error returned by [`Client::execute`].
#[derive(Debug, Error)]
pub enum Error<T> {
    /// The executor failed to send the request or decode the response.
    #[error(transparent)]
    Executor(#[from] T),

    /// Telegram returned an `ok: false` Bot API response.
    #[error("telegram bot api returned error code: {error_code}, description: {description}")]
    Api {
        /// Telegram Bot API error code.
        error_code: u16,
        /// Human-readable Telegram Bot API error description.
        description: String,
    },
}

impl<E> Client<E>
where
    E: ClientExecutor,
{
    /// Creates a client from an executor and a bot token.
    ///
    /// The token is inserted into request URLs in the form
    /// `https://api.telegram.org/bot<TOKEN>/<METHOD>`.
    pub fn new(executor: E, token: impl Into<String>) -> Self {
        Self {
            executor,
            token: token.into(),
        }
    }

    /// Executes a typed Telegram Bot API method.
    ///
    /// The method's [`Method::NAME`] selects the Telegram endpoint and the
    /// method's associated response type determines the decoded return value.
    pub async fn execute<M>(&mut self, method: M) -> Result<M::Response, Error<E::Error>>
    where
        M: Method + Send,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Response<T> {
            Success {
                #[allow(dead_code)]
                ok: monostate::MustBe!(true),
                result: T,
            },
            Failure {
                #[allow(dead_code)]
                ok: monostate::MustBe!(false),
                error_code: u16,
                description: String,
            },
        }

        let uri = format!(
            "https://api.telegram.org/bot{token}/{name}",
            token = self.token,
            name = M::NAME,
        );

        let uri = Uri::try_from(uri).unwrap();

        let response = self
            .executor
            .request::<M, Response<M::Response>>(uri, method)
            .await?;

        match response {
            Response::Success { ok: _, result } => Ok(result),

            Response::Failure {
                ok: _,
                error_code,
                description,
            } => Err(Error::Api {
                error_code,
                description,
            }),
        }
    }
}
