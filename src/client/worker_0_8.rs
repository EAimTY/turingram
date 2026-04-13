//! [`ClientExecutor`](crate::client::ClientExecutor) implementation for
//! Cloudflare Workers using `worker` 0.8.

use super::ClientExecutor;
use http::{HeaderValue, Method, Request, Uri, header};
use serde::{Deserialize, Serialize};
use worker_0_8::{Error as WorkerError, send::SendFuture};
use worker_helper_0_8::{
    Fetch,
    body::{BodyExt as _, Json, JsonBodyError},
};

/// Error returned while sending a Worker request or decoding its JSON body.
pub type Error = JsonBodyError<WorkerError>;

/// Cloudflare Workers HTTP executor.
///
/// This executor sends Bot API requests with `worker::send::SendFuture` and
/// decodes JSON responses through `worker-helper`.
pub struct Executor {
    _priv: (),
}

impl Executor {
    /// Creates a Cloudflare Workers executor.
    pub fn new() -> Self {
        Self { _priv: () }
    }
}

impl ClientExecutor for Executor {
    type Error = Error;

    async fn request<T, U>(&mut self, uri: Uri, payload: T) -> Result<U, Self::Error>
    where
        T: Serialize + Send,
        U: for<'a> Deserialize<'a>,
    {
        let request = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            )
            .body(Json::new(payload))
            .unwrap();

        SendFuture::new(Fetch(request).send())
            .await?
            .into_body()
            .json()
            .await
    }
}
