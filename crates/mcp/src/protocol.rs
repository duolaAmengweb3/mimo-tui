//! High-level MCP protocol types.

use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: &str = "2024-11-05";

#[derive(Debug, Clone, Serialize)]
pub(crate) struct InitializeParams {
    pub protocol_version: &'static str,
    pub capabilities: ClientCapabilities,
    pub client_info: ClientInfo,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ClientCapabilities {
    pub tools: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct ClientInfo {
    pub name: &'static str,
    pub version: &'static str,
}

/// What the server tells us during initialize.
#[derive(Debug, Clone, Deserialize)]
pub struct InitializeResult {
    #[serde(default, rename = "protocolVersion")]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub capabilities: ServerCapabilities,
    #[serde(default, rename = "serverInfo")]
    pub server_info: Option<ServerInfo>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ServerCapabilities {
    #[serde(default)]
    pub tools: Option<serde_json::Value>,
    #[serde(default)]
    pub resources: Option<serde_json::Value>,
    #[serde(default)]
    pub prompts: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

/// One tool advertised by an MCP server.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct McpTool {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default, rename = "inputSchema")]
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct ListToolsResult {
    pub tools: Vec<McpTool>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct CallToolResult {
    pub content: Vec<ContentItem>,
    #[serde(default, rename = "isError")]
    pub is_error: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum ContentItem {
    Text { text: String },
    #[serde(other)]
    Other,
}

impl CallToolResult {
    pub(crate) fn into_text(self) -> String {
        let mut out = String::new();
        for item in self.content {
            if let ContentItem::Text { text } = item {
                if !out.is_empty() {
                    out.push('\n');
                }
                out.push_str(&text);
            }
        }
        if self.is_error && !out.is_empty() {
            out.insert_str(0, "[mcp error] ");
        }
        out
    }
}
