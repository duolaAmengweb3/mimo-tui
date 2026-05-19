//! Error types for the Anthropic protocol client.

use thiserror::Error;

/// All errors that can come out of this crate.
#[derive(Debug, Error)]
pub enum AnthropicError {
    /// Underlying HTTP transport error (network / TLS / DNS).
    #[error("HTTP transport error: {0}")]
    Http(#[from] reqwest::Error),

    /// The API returned a non-2xx status with an error envelope.
    #[error("API error {status}: {message}")]
    Api {
        status: u16,
        message: String,
        body: String,
    },

    /// Server returned 401.
    #[error("Authentication failed: invalid API key or missing permissions")]
    Unauthorized,

    /// Server returned 429.
    #[error("Rate limit hit; retry-after {retry_after_secs:?}s")]
    RateLimit { retry_after_secs: Option<u64> },

    /// Server returned 5xx.
    #[error("Server error {status}: {body}")]
    Server { status: u16, body: String },

    /// JSON deserialization failed.
    #[error("JSON deserialize error: {0}")]
    Decode(#[from] serde_json::Error),

    /// SSE stream ended in an unexpected way.
    #[error("Stream error: {0}")]
    Stream(String),

    /// URL parse error.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Other / generic.
    #[error("{0}")]
    Other(String),
}

impl AnthropicError {
    /// True if a retry would plausibly succeed.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AnthropicError::Http(_)
                | AnthropicError::RateLimit { .. }
                | AnthropicError::Server { .. }
                | AnthropicError::Stream(_)
        )
    }
}

pub type Result<T> = std::result::Result<T, AnthropicError>;
