//! `web_fetch` — fetch a URL and return text content.

use std::time::Duration;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use crate::{Tool, ToolContext, ToolResult};

#[derive(Deserialize)]
struct Input {
    url: String,
    #[serde(default = "default_max_kb")]
    max_kb: usize,
}

fn default_max_kb() -> usize {
    256
}

pub struct WebFetch;

#[async_trait]
impl Tool for WebFetch {
    fn name(&self) -> &'static str {
        "web_fetch"
    }

    fn description(&self) -> &'static str {
        "Fetch a URL via HTTPS and return its text body (truncated to 256KB by default). \
         For HTML, you'll receive the raw markup — extract what you need."
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "url":    { "type": "string", "format": "uri" },
                "max_kb": { "type": "integer", "default": 256 }
            },
            "required": ["url"]
        })
    }

    async fn run(
        &self,
        _ctx: &ToolContext,
        input: serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        let Input { url, max_kb } = serde_json::from_value(input)?;
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(concat!("mimo-tui/", env!("CARGO_PKG_VERSION")))
            .build()?;
        let resp = match client.get(&url).send().await {
            Ok(r) => r,
            Err(e) => return Ok(ToolResult::err(format!("fetch error: {}", e))),
        };
        let status = resp.status();
        let body = match resp.text().await {
            Ok(b) => b,
            Err(e) => return Ok(ToolResult::err(format!("body decode: {}", e))),
        };

        let max_bytes = max_kb * 1024;
        let mut truncated_body = body
            .chars()
            .scan(0usize, |acc, c| {
                let n = c.len_utf8();
                if *acc + n > max_bytes {
                    None
                } else {
                    *acc += n;
                    Some(c)
                }
            })
            .collect::<String>();
        if truncated_body.len() < body.len() {
            truncated_body.push_str("\n\n... (truncated)");
        }

        let header = format!("HTTP {}\n\n", status.as_u16());
        Ok(ToolResult::ok(format!("{}{}", header, truncated_body)))
    }
}
