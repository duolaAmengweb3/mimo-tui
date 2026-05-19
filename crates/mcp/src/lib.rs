//! Model Context Protocol (MCP) client.
//!
//! Currently implements the **stdio transport** only — the most common transport
//! for shipped MCP servers (e.g. `npx -y @modelcontextprotocol/server-filesystem`).
//! `streamable-http` and `sse` will land in a follow-up.
//!
//! Spec: <https://modelcontextprotocol.io/specification>

pub mod http;
pub mod jsonrpc;
pub mod protocol;
pub mod stdio;

pub use http::{HttpServer, HttpServerConfig};
pub use protocol::{McpTool, ServerCapabilities, ServerInfo};
pub use stdio::{StdioServer, StdioServerConfig};

use anyhow::Result;

/// Generic MCP client interface (so we can swap transports later).
#[async_trait::async_trait]
pub trait McpClient: Send + Sync {
    /// Run the protocol handshake.
    async fn initialize(&self) -> Result<ServerInfo>;

    /// List the tools advertised by the server.
    async fn list_tools(&self) -> Result<Vec<McpTool>>;

    /// Invoke a tool. Returns the textual result content.
    async fn call_tool(&self, name: &str, args: serde_json::Value) -> Result<String>;

    /// Best-effort graceful shutdown.
    async fn shutdown(&self) -> Result<()>;
}
