//! MCP（Model Context Protocol）客户端
//!
//! 支持三种 transport：
//! - stdio
//! - SSE
//! - streamable HTTP
//!
//! 兼容 Claude Code / Codex / Cursor 已有的 MCP server 生态。
//!
//! 待实现：
//! - `transport::stdio`
//! - `transport::sse`
//! - `transport::http`
//! - `protocol` —— JSON-RPC 2.0 + initialize / tools / resources / prompts
//! - `registry` —— 已安装 MCP server 管理
//! - `install`  —— `/mcp install <github-repo>`
