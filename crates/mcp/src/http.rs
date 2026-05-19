//! Streamable HTTP transport for MCP.
//!
//! Most third-party MCP servers run over stdio. A growing minority expose an
//! HTTP endpoint instead — POST JSON-RPC requests, get back either:
//!   - JSON response (single request/response), or
//!   - text/event-stream SSE stream (long-running tool with streamed deltas).
//!
//! This client supports the request/response shape and parses the headers but
//! does NOT yet stream tool deltas back to the agent. Tool calls block until
//! the server's final result lands.

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::debug;

use crate::jsonrpc::{Request, Response};
use crate::protocol::{
    CallToolResult, ClientCapabilities, ClientInfo, InitializeParams, InitializeResult,
    ListToolsResult, McpTool, ServerInfo, PROTOCOL_VERSION,
};
use crate::McpClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpServerConfig {
    pub url: String,
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
}

pub struct HttpServer {
    http: reqwest::Client,
    name: String,
    url: String,
    headers: HeaderMap,
    next_id: Arc<Mutex<u64>>,
}

impl HttpServer {
    pub fn new(name: impl Into<String>, config: HttpServerConfig) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .user_agent(concat!("mimo-tui/", env!("CARGO_PKG_VERSION")))
            .build()
            .context("build http client")?;

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        // Accept both JSON and SSE; servers pick the appropriate one per request.
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/event-stream"),
        );
        for (k, v) in &config.headers {
            if let (Ok(name), Ok(val)) = (
                reqwest::header::HeaderName::from_bytes(k.as_bytes()),
                HeaderValue::from_str(v),
            ) {
                headers.insert(name, val);
            }
        }

        Ok(Self {
            http,
            name: name.into(),
            url: config.url,
            headers,
            next_id: Arc::new(Mutex::new(1)),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    async fn send_request(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let id = {
            let mut g = self.next_id.lock().await;
            let id = *g;
            *g += 1;
            id
        };
        let req = Request::new(id, method, params);
        debug!(server = %self.name, method = method, "POST mcp");

        let resp = self
            .http
            .post(&self.url)
            .headers(self.headers.clone())
            .json(&req)
            .send()
            .await
            .with_context(|| format!("POST {}", self.url))?;

        let status = resp.status();
        let content_type = resp
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("HTTP {} from MCP server: {}", status, body));
        }

        // SSE stream → drain to find the final `result` event.
        if content_type.starts_with("text/event-stream") {
            let body = resp.text().await?;
            for chunk in body.split("\n\n") {
                let mut data_lines: Vec<&str> = Vec::new();
                for line in chunk.lines() {
                    if let Some(d) = line.strip_prefix("data: ") {
                        data_lines.push(d);
                    } else if let Some(d) = line.strip_prefix("data:") {
                        data_lines.push(d);
                    }
                }
                if data_lines.is_empty() {
                    continue;
                }
                let payload = data_lines.join("\n");
                if let Ok(parsed) = serde_json::from_str::<Response>(&payload) {
                    if parsed.id.as_u64() == Some(id) {
                        if let Some(err) = parsed.error {
                            return Err(anyhow!("mcp error {}: {}", err.code, err.message));
                        }
                        return Ok(parsed.result.unwrap_or(serde_json::Value::Null));
                    }
                }
            }
            return Err(anyhow!("SSE stream ended without a response for id {}", id));
        }

        // Plain JSON response.
        let parsed: Response = resp.json().await?;
        if let Some(err) = parsed.error {
            return Err(anyhow!("mcp error {}: {}", err.code, err.message));
        }
        Ok(parsed.result.unwrap_or(serde_json::Value::Null))
    }
}

#[async_trait]
impl McpClient for HttpServer {
    async fn initialize(&self) -> Result<ServerInfo> {
        let params = InitializeParams {
            protocol_version: PROTOCOL_VERSION,
            capabilities: ClientCapabilities {
                tools: serde_json::json!({}),
            },
            client_info: ClientInfo {
                name: "mimo-tui",
                version: env!("CARGO_PKG_VERSION"),
            },
        };
        let result = self
            .send_request("initialize", Some(serde_json::to_value(params)?))
            .await?;
        let parsed: InitializeResult = serde_json::from_value(result)?;
        // Best effort: send the initialized notification (server may or may not require it).
        let _ = self
            .http
            .post(&self.url)
            .headers(self.headers.clone())
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "method": "notifications/initialized"
            }))
            .send()
            .await;
        Ok(parsed.server_info.unwrap_or(ServerInfo {
            name: self.name.clone(),
            version: "unknown".to_string(),
        }))
    }

    async fn list_tools(&self) -> Result<Vec<McpTool>> {
        let v = self.send_request("tools/list", None).await?;
        let parsed: ListToolsResult = serde_json::from_value(v)?;
        Ok(parsed.tools)
    }

    async fn call_tool(&self, name: &str, args: serde_json::Value) -> Result<String> {
        let params = serde_json::json!({
            "name": name,
            "arguments": args,
        });
        let v = self.send_request("tools/call", Some(params)).await?;
        let parsed: CallToolResult = serde_json::from_value(v)?;
        Ok(parsed.into_text())
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}
