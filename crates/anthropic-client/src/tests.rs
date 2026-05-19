//! Unit + integration tests.
//!
//! Integration tests against real MiMo Token Plan are gated behind the
//! `MIMO_API_KEY` env var so they don't run in vanilla CI without secrets.

use super::*;

// ============================================================================
// Unit tests (no network)
// ============================================================================

#[test]
fn region_base_urls_are_correct() {
    assert_eq!(
        Region::Cn.base_url(),
        "https://token-plan-cn.xiaomimimo.com/anthropic"
    );
    assert_eq!(
        Region::Sgp.base_url(),
        "https://token-plan-sgp.xiaomimimo.com/anthropic"
    );
    assert_eq!(
        Region::Ams.base_url(),
        "https://token-plan-ams.xiaomimimo.com/anthropic"
    );
}

#[test]
fn deserialize_real_mimo_response() {
    // From actual MiMo Token Plan response captured in 调研/03-API实测结果.md.
    let json = r#"{
        "id": "517aa5f2c2c04029add0a4a49ebb94b6",
        "type": "message",
        "role": "assistant",
        "model": "mimo-v2.5-pro",
        "stop_reason": "end_turn",
        "content": [
            { "type": "text", "text": "我是 MiMo-v2.5-pro" },
            { "type": "thinking", "thinking": "The user asked who I am.", "signature": "" }
        ],
        "usage": {
            "input_tokens": 69,
            "output_tokens": 47,
            "cache_read_input_tokens": 192
        }
    }"#;

    let resp: MessagesResponse = serde_json::from_str(json).expect("should deserialize");
    assert_eq!(resp.model, "mimo-v2.5-pro");
    assert!(matches!(resp.stop_reason, Some(StopReason::EndTurn)));
    assert_eq!(resp.content.len(), 2);
    assert_eq!(resp.text(), "我是 MiMo-v2.5-pro");
    assert_eq!(
        resp.thinking(),
        Some("The user asked who I am.".to_string())
    );
    assert_eq!(resp.usage.input_tokens, 69);
    assert_eq!(resp.usage.cache_read_input_tokens, 192);
}

#[test]
fn deserialize_tool_use_block() {
    let json = r#"{
        "type": "tool_use",
        "id": "toolu_01abc",
        "name": "read_file",
        "input": { "path": "src/main.rs", "max_lines": 50 }
    }"#;
    let block: ContentBlock = serde_json::from_str(json).unwrap();
    match block {
        ContentBlock::ToolUse { id, name, input } => {
            assert_eq!(id, "toolu_01abc");
            assert_eq!(name, "read_file");
            assert_eq!(input["path"], "src/main.rs");
        }
        _ => panic!("expected ToolUse"),
    }
}

#[test]
fn serialize_request_omits_none() {
    let req = MessagesRequest::new("mimo-v2.5-pro", 100).user("hi");
    let json = serde_json::to_string(&req).unwrap();
    // No nulls or empty optional fields in the wire format.
    assert!(!json.contains("\"system\":null"));
    assert!(!json.contains("\"tools\":null"));
    assert!(json.contains("\"max_tokens\":100"));
    assert!(json.contains("\"messages\""));
}

#[test]
fn usage_cache_ratio() {
    let u = Usage {
        input_tokens: 100,
        output_tokens: 50,
        cache_creation_input_tokens: 0,
        cache_read_input_tokens: 900,
    };
    assert!((u.cache_hit_ratio() - 0.9).abs() < 1e-6);
}

#[test]
fn stream_event_deserialization() {
    let payload = r#"{ "type": "ping" }"#;
    let ev: StreamEvent = serde_json::from_str(payload).unwrap();
    assert!(matches!(ev, StreamEvent::Ping));

    let delta = r#"{
        "type": "content_block_delta",
        "index": 0,
        "delta": { "type": "text_delta", "text": "hello" }
    }"#;
    let ev: StreamEvent = serde_json::from_str(delta).unwrap();
    if let StreamEvent::ContentBlockDelta { index, delta } = ev {
        assert_eq!(index, 0);
        if let BlockDelta::TextDelta { text } = delta {
            assert_eq!(text, "hello");
        } else {
            panic!("wrong delta variant");
        }
    } else {
        panic!("wrong event variant");
    }
}

// ============================================================================
// Integration tests against real MiMo API
// ============================================================================
//
// Run with:
//   MIMO_API_KEY=tp-xxx cargo test --package mimo-tui-anthropic-client \
//     --features integration -- --ignored --test-threads=1

#[tokio::test]
#[ignore = "needs MIMO_API_KEY env var + network"]
async fn live_simple_message() {
    let key = std::env::var("MIMO_API_KEY").expect("MIMO_API_KEY must be set");
    let client = Client::new(key, Region::Sgp);
    let resp = client
        .messages(
            MessagesRequest::new("mimo-v2.5-pro", 120)
                .user("Reply with the single word: OK"),
        )
        .await
        .expect("request should succeed");

    let text = resp.text();
    assert!(!text.is_empty(), "response should have text");
    assert!(resp.usage.input_tokens > 0);
    assert!(resp.usage.output_tokens > 0);
}

#[tokio::test]
#[ignore = "needs MIMO_API_KEY env var + network"]
async fn live_streaming() {
    use futures::StreamExt;

    let key = std::env::var("MIMO_API_KEY").expect("MIMO_API_KEY must be set");
    let client = Client::new(key, Region::Sgp);
    let resp = client
        .messages_stream_raw(
            MessagesRequest::new("mimo-v2.5-pro", 120).user("Count from 1 to 5"),
        )
        .await
        .expect("stream request should succeed");

    let mut events = Box::pin(stream::events(resp));
    let mut seen_message_start = false;
    let mut seen_message_stop = false;
    while let Some(ev) = events.next().await {
        match ev.expect("stream event should parse") {
            StreamEvent::MessageStart { .. } => seen_message_start = true,
            StreamEvent::MessageStop => seen_message_stop = true,
            _ => {}
        }
    }
    assert!(seen_message_start);
    assert!(seen_message_stop);
}

#[tokio::test]
#[ignore = "needs MIMO_API_KEY env var + network"]
async fn live_streaming_collect() {
    let key = std::env::var("MIMO_API_KEY").expect("MIMO_API_KEY must be set");
    let client = Client::new(key, Region::Sgp);
    let resp = client
        .messages_stream_raw(
            MessagesRequest::new("mimo-v2.5-pro", 200).user("Say hi in one sentence."),
        )
        .await
        .expect("stream request should succeed");

    let final_resp = stream::collect(resp).await.expect("collect should succeed");
    assert!(!final_resp.text().is_empty());
}

#[tokio::test]
#[ignore = "needs MIMO_API_KEY env var + network"]
async fn live_invalid_key() {
    let client = Client::new("tp-definitely-invalid".to_string(), Region::Sgp);
    let result = client
        .messages(MessagesRequest::new("mimo-v2.5-pro", 50).user("hi"))
        .await;
    match result {
        Err(AnthropicError::Unauthorized) => {} // expected
        Err(other) => panic!("expected Unauthorized, got {:?}", other),
        Ok(_) => panic!("expected auth failure"),
    }
}
