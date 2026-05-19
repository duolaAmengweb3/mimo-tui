//! SSE stream decoder for Anthropic Messages API.
//!
//! Anthropic streams as `text/event-stream` with `event: <type>` + `data: <json>`
//! pairs separated by blank lines. We accept the `data:` JSON and dispatch by its
//! `type` field (we ignore the `event:` line since it duplicates the type).

use eventsource_stream::Eventsource;
use futures::Stream;
use tracing::warn;

use crate::error::AnthropicError;
use crate::types::StreamEvent;

/// Decode a streaming response into a stream of [`StreamEvent`]s.
pub fn events(resp: reqwest::Response) -> impl Stream<Item = Result<StreamEvent, AnthropicError>> {
    use futures::StreamExt;

    resp.bytes_stream()
        .eventsource()
        .filter_map(|event| async move {
            match event {
                Ok(e) => {
                    // Filter out "ping" comments and empty data lines.
                    if e.data.is_empty() {
                        return None;
                    }
                    match serde_json::from_str::<StreamEvent>(&e.data) {
                        Ok(parsed) => Some(Ok(parsed)),
                        Err(err) => {
                            warn!(?err, raw = %e.data, "stream decode failed");
                            Some(Err(AnthropicError::Decode(err)))
                        }
                    }
                }
                Err(err) => Some(Err(AnthropicError::Stream(err.to_string()))),
            }
        })
}

/// Convenience: collect a full streaming response into a single
/// [`crate::types::MessagesResponse`].
pub async fn collect(
    resp: reqwest::Response,
) -> Result<crate::types::MessagesResponse, AnthropicError> {
    use futures::StreamExt;

    use crate::types::{BlockDelta, ContentBlock, MessagesResponse, Role, StreamEvent};

    let mut stream = Box::pin(events(resp));
    let mut response: Option<MessagesResponse> = None;
    let mut blocks: Vec<ContentBlock> = Vec::new();
    // For each open block index, we maintain a string buffer for text/thinking
    // and accumulate input JSON for tool_use.
    let mut text_buf: Vec<String> = Vec::new();
    let mut thinking_buf: Vec<String> = Vec::new();
    let mut input_json_buf: Vec<String> = Vec::new();
    let mut signature_buf: Vec<String> = Vec::new();

    while let Some(item) = stream.next().await {
        match item? {
            StreamEvent::MessageStart { message } => {
                response = Some(message);
            }
            StreamEvent::ContentBlockStart {
                index,
                content_block,
            } => {
                let idx = index as usize;
                while blocks.len() <= idx {
                    blocks.push(ContentBlock::Text {
                        text: String::new(),
                        cache_control: None,
                    });
                    text_buf.push(String::new());
                    thinking_buf.push(String::new());
                    input_json_buf.push(String::new());
                    signature_buf.push(String::new());
                }
                blocks[idx] = content_block;
            }
            StreamEvent::ContentBlockDelta { index, delta } => {
                let idx = index as usize;
                while blocks.len() <= idx {
                    blocks.push(ContentBlock::Text {
                        text: String::new(),
                        cache_control: None,
                    });
                    text_buf.push(String::new());
                    thinking_buf.push(String::new());
                    input_json_buf.push(String::new());
                    signature_buf.push(String::new());
                }
                match delta {
                    BlockDelta::TextDelta { text } => text_buf[idx].push_str(&text),
                    BlockDelta::ThinkingDelta { thinking } => thinking_buf[idx].push_str(&thinking),
                    BlockDelta::InputJsonDelta { partial_json } => {
                        input_json_buf[idx].push_str(&partial_json)
                    }
                    BlockDelta::SignatureDelta { signature } => {
                        signature_buf[idx].push_str(&signature)
                    }
                }
            }
            StreamEvent::ContentBlockStop { index } => {
                let idx = index as usize;
                if let Some(block) = blocks.get_mut(idx) {
                    match block {
                        ContentBlock::Text { text, .. } => {
                            text.push_str(&text_buf[idx]);
                        }
                        ContentBlock::Thinking {
                            thinking,
                            signature,
                        } => {
                            thinking.push_str(&thinking_buf[idx]);
                            if !signature_buf[idx].is_empty() {
                                *signature = signature_buf[idx].clone();
                            }
                        }
                        ContentBlock::ToolUse { input, .. } => {
                            if !input_json_buf[idx].is_empty() {
                                if let Ok(v) =
                                    serde_json::from_str::<serde_json::Value>(&input_json_buf[idx])
                                {
                                    *input = v;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            StreamEvent::MessageDelta { delta, usage } => {
                if let Some(r) = response.as_mut() {
                    if delta.stop_reason.is_some() {
                        r.stop_reason = delta.stop_reason;
                    }
                    if delta.stop_sequence.is_some() {
                        r.stop_sequence = delta.stop_sequence;
                    }
                    r.usage.output_tokens = usage.output_tokens;
                    if usage.cache_read_input_tokens > 0 {
                        r.usage.cache_read_input_tokens = usage.cache_read_input_tokens;
                    }
                    if usage.cache_creation_input_tokens > 0 {
                        r.usage.cache_creation_input_tokens = usage.cache_creation_input_tokens;
                    }
                }
            }
            StreamEvent::MessageStop => break,
            StreamEvent::Ping => {}
            StreamEvent::Error { error } => {
                return Err(AnthropicError::Stream(format!(
                    "server error event: {}",
                    error
                )));
            }
        }
    }

    let mut r = response.unwrap_or(crate::types::MessagesResponse {
        id: String::new(),
        kind: "message".to_string(),
        role: Role::Assistant,
        model: String::new(),
        content: Vec::new(),
        stop_reason: None,
        stop_sequence: None,
        usage: Default::default(),
    });
    r.content = blocks;
    Ok(r)
}
