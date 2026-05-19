//! HTTP client for the Anthropic Messages API (used with MiMo Token Plan).
//!
//! ```no_run
//! use mimo_tui_anthropic_client::{Client, MessagesRequest, Region};
//!
//! # async fn demo() -> anyhow::Result<()> {
//! let client = Client::new("tp-xxx".into(), Region::Sgp);
//! let resp = client
//!     .messages(
//!         MessagesRequest::new("mimo-v2.5-pro", 1024)
//!             .user("Say hi"),
//!     )
//!     .await?;
//! println!("{}", resp.text());
//! # Ok(())
//! # }
//! ```

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use tracing::{debug, warn};

use crate::error::{AnthropicError, Result};
use crate::types::{MessagesRequest, MessagesResponse};

const ANTHROPIC_VERSION: &str = "2023-06-01";
const DEFAULT_TIMEOUT_SECS: u64 = 300;

/// MiMo Token Plan regional clusters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    Cn,
    Sgp,
    Ams,
    /// Use any custom base URL (advanced).
    Custom,
}

impl Region {
    pub fn base_url(self) -> &'static str {
        match self {
            Region::Cn => "https://token-plan-cn.xiaomimimo.com/anthropic",
            Region::Sgp => "https://token-plan-sgp.xiaomimimo.com/anthropic",
            Region::Ams => "https://token-plan-ams.xiaomimimo.com/anthropic",
            Region::Custom => "",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Region::Cn => "CN",
            Region::Sgp => "SGP",
            Region::Ams => "AMS",
            Region::Custom => "custom",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
    anthropic_version: String,
}

impl Client {
    /// Build a client for one of the three official MiMo Token Plan clusters.
    pub fn new(api_key: String, region: Region) -> Self {
        Self::with_base_url(api_key, region.base_url().to_string())
    }

    /// Build a client pointing at any custom base URL (must include `/anthropic`).
    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .user_agent(concat!("mimo-tui/", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("reqwest client should build");
        Self {
            http,
            base_url,
            api_key,
            anthropic_version: ANTHROPIC_VERSION.to_string(),
        }
    }

    pub fn with_anthropic_version(mut self, v: impl Into<String>) -> Self {
        self.anthropic_version = v.into();
        self
    }

    /// Non-streaming `POST /v1/messages`.
    pub async fn messages(&self, req: MessagesRequest) -> Result<MessagesResponse> {
        let mut req = req;
        req.stream = Some(false);

        let url = format!("{}/v1/messages", self.base_url);
        debug!(?url, model = %req.model, "POST messages");

        let resp = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(&req)
            .send()
            .await?;

        let status = resp.status();
        let body = resp.text().await?;

        if status.is_success() {
            let parsed: MessagesResponse = serde_json::from_str(&body)?;
            Ok(parsed)
        } else {
            Err(map_error(status.as_u16(), body))
        }
    }

    /// Streaming `POST /v1/messages` with `stream: true`. Returns the raw
    /// `reqwest::Response`; pair with [`crate::stream::events`] to get
    /// decoded `StreamEvent`s.
    pub async fn messages_stream_raw(&self, req: MessagesRequest) -> Result<reqwest::Response> {
        let mut req = req;
        req.stream = Some(true);

        let url = format!("{}/v1/messages", self.base_url);
        debug!(?url, model = %req.model, "POST messages (stream)");

        let resp = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(&req)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let body = resp.text().await?;
            return Err(map_error(status, body));
        }
        Ok(resp)
    }

    fn headers(&self) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        h.insert(
            "x-api-key",
            HeaderValue::from_str(&self.api_key).unwrap_or_else(|_| {
                warn!("api_key contains invalid header characters");
                HeaderValue::from_static("invalid")
            }),
        );
        h.insert(
            "anthropic-version",
            HeaderValue::from_str(&self.anthropic_version).unwrap_or_else(|_| {
                HeaderValue::from_static(ANTHROPIC_VERSION)
            }),
        );
        h
    }
}

fn map_error(status: u16, body: String) -> AnthropicError {
    // Try to extract message field from JSON envelope.
    let message = serde_json::from_str::<serde_json::Value>(&body)
        .ok()
        .and_then(|v| {
            v.get("error")
                .and_then(|e| e.get("message"))
                .and_then(|m| m.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| body.chars().take(200).collect());

    match status {
        401 => AnthropicError::Unauthorized,
        429 => AnthropicError::RateLimit { retry_after_secs: None },
        500..=599 => AnthropicError::Server { status, body },
        _ => AnthropicError::Api {
            status,
            message,
            body,
        },
    }
}
