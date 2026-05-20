//! Bridges MCP servers into the agent's tool surface.
//!
//! - Reads `~/.mimo/mcp.json` (a map of server-name → StdioServerConfig)
//! - Spawns each server, runs `initialize` + `tools/list`
//! - Exposes a single Tool trait wrapper per MCP tool so the agent sees them
//!   as normal native tools

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{Context, Result};
use mimo_tui_mcp::{HttpServer, HttpServerConfig, McpClient, McpTool, StdioServer, StdioServerConfig};
use mimo_tui_tools::{Tool, ToolRegistry, ToolResult};
use tracing::{info, warn};

use crate::paths;

/// A single MCP server entry in `~/.mimo/mcp.json`. Either stdio (subprocess)
/// or http (remote endpoint).
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum ServerEntry {
    Http(HttpServerConfig),
    Stdio(StdioServerConfig),
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct McpConfig {
    #[serde(default)]
    pub servers: HashMap<String, ServerEntry>,
}

/// Lifetime owner for spawned MCP servers + registered tools.
pub struct McpHub {
    pub servers: Vec<Arc<dyn McpClient>>,
}

impl McpHub {
    pub async fn init(registry: &mut ToolRegistry) -> Result<Self> {
        let config_path = paths::mimo_dir()?.join("mcp.json");
        if !config_path.exists() {
            return Ok(Self {
                servers: Vec::new(),
            });
        }
        let raw = std::fs::read_to_string(&config_path)
            .with_context(|| format!("read {}", config_path.display()))?;
        let cfg: McpConfig = serde_json::from_str(&raw).context("parse mcp.json")?;

        let mut servers: Vec<Arc<dyn McpClient>> = Vec::new();
        for (name, server_cfg) in cfg.servers {
            let spawn_result: Result<Arc<dyn McpClient>> = match server_cfg {
                ServerEntry::Stdio(mut cfg) => {
                    for arg in cfg.args.iter_mut() {
                        *arg = expand_home(arg);
                    }
                    StdioServer::spawn(name.clone(), cfg)
                        .await
                        .map(|s| Arc::new(s) as Arc<dyn McpClient>)
                }
                ServerEntry::Http(cfg) => {
                    HttpServer::new(name.clone(), cfg).map(|s| Arc::new(s) as Arc<dyn McpClient>)
                }
            };
            match spawn_result {
                Ok(client) => {
                    match client.initialize().await {
                        Ok(info) => {
                            info!(server = %name, version = %info.version, "MCP initialized");
                        }
                        Err(e) => {
                            warn!(server = %name, err = ?e, "MCP initialize failed");
                            continue;
                        }
                    }
                    match client.list_tools().await {
                        Ok(tools) => {
                            for t in tools {
                                let tool_name = format!("mcp__{}__{}", name, t.name);
                                let wrapped: Arc<dyn Tool> = Arc::new(McpToolAdapter {
                                    full_name: leak(tool_name.clone()),
                                    description: leak(t.description.clone()),
                                    schema: t.input_schema.clone(),
                                    inner_name: t.name.clone(),
                                    client: client.clone(),
                                });
                                registry.insert(wrapped);
                            }
                        }
                        Err(e) => warn!(server = %name, err = ?e, "MCP list_tools failed"),
                    }
                    servers.push(client);
                }
                Err(e) => {
                    warn!(server = %name, err = ?e, "failed to spawn MCP server");
                }
            }
        }
        Ok(Self { servers })
    }
}

/// Leak a string to get a `'static` reference — fine for boot-time tool names.
fn leak(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/// Expand leading `~` and `$HOME` in a path-like arg so users can write
/// `"$HOME"` or `"~/projects"` in `mcp.json` without hardcoding their username.
fn expand_home(arg: &str) -> String {
    let Ok(home) = paths::home_dir() else {
        return arg.to_string();
    };
    let home_str = home.to_string_lossy();
    if arg == "~" || arg == "$HOME" {
        return home_str.into_owned();
    }
    if let Some(rest) = arg.strip_prefix("~/") {
        return format!("{}/{}", home_str, rest);
    }
    if let Some(rest) = arg.strip_prefix("$HOME/") {
        return format!("{}/{}", home_str, rest);
    }
    arg.to_string()
}

struct McpToolAdapter {
    full_name: &'static str,
    description: &'static str,
    schema: serde_json::Value,
    inner_name: String,
    client: Arc<dyn McpClient>,
}

#[async_trait::async_trait]
impl Tool for McpToolAdapter {
    fn name(&self) -> &'static str {
        self.full_name
    }

    fn description(&self) -> &'static str {
        self.description
    }

    fn input_schema(&self) -> serde_json::Value {
        self.schema.clone()
    }

    async fn run(
        &self,
        _ctx: &mimo_tui_tools::ToolContext,
        input: serde_json::Value,
    ) -> Result<ToolResult> {
        match self.client.call_tool(&self.inner_name, input).await {
            Ok(text) => Ok(ToolResult::ok(text)),
            Err(e) => Ok(ToolResult::err(format!("mcp tool error: {}", e))),
        }
    }
}

/// For convenience: list configured MCP servers without spawning.
pub fn list_configured() -> Result<Vec<(String, ServerEntry)>> {
    let p = paths::mimo_dir()?.join("mcp.json");
    if !p.exists() {
        return Ok(Vec::new());
    }
    let raw = std::fs::read_to_string(&p)?;
    let cfg: McpConfig = serde_json::from_str(&raw)?;
    Ok(cfg.servers.into_iter().collect())
}

/// Used by McpToolAdapter to point to McpTool (avoid `unused`).
#[allow(dead_code)]
fn _force_use(_t: McpTool) {}
