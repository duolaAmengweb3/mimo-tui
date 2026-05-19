//! Lightweight Anthropic Messages API client for the MiMo Token Plan.
//!
//! ## Usage
//!
//! Non-streaming:
//! ```no_run
//! use mimo_tui_anthropic_client::{Client, MessagesRequest, Region};
//!
//! # async fn demo() -> anyhow::Result<()> {
//! let client = Client::new(std::env::var("MIMO_API_KEY")?, Region::Sgp);
//! let resp = client
//!     .messages(MessagesRequest::new("mimo-v2.5-pro", 1024).user("Hello"))
//!     .await?;
//! println!("{}", resp.text());
//! # Ok(()) }
//! ```
//!
//! Streaming:
//! ```no_run
//! use mimo_tui_anthropic_client::{Client, MessagesRequest, Region, stream};
//! use futures::StreamExt;
//!
//! # async fn demo() -> anyhow::Result<()> {
//! let client = Client::new(std::env::var("MIMO_API_KEY")?, Region::Sgp);
//! let resp = client
//!     .messages_stream_raw(MessagesRequest::new("mimo-v2.5-pro", 1024).user("Hi"))
//!     .await?;
//! let mut events = Box::pin(stream::events(resp));
//! while let Some(ev) = events.next().await {
//!     println!("{:?}", ev?);
//! }
//! # Ok(()) }
//! ```

pub mod client;
pub mod error;
pub mod stream;
pub mod types;

pub use client::{Client, Region};
pub use error::{AnthropicError, Result};
pub use types::{
    BlockDelta, CacheControl, ContentBlock, ImageSource, Message, MessageContent,
    MessagesRequest, MessagesResponse, Role, StopReason, StreamEvent, SystemPrompt,
    Tool, ToolChoice, ToolResultContent, Usage,
};

#[cfg(test)]
mod tests;
