//! Stdio transport: spawn an MCP server process, exchange line-delimited JSON-RPC.

use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, Command};
use tokio::sync::{oneshot, Mutex};
use tracing::{debug, warn};

use crate::jsonrpc::{Notification, Request, Response};
use crate::protocol::{
    CallToolResult, ClientCapabilities, ClientInfo, InitializeParams, InitializeResult,
    ListToolsResult, McpTool, ServerInfo, PROTOCOL_VERSION,
};
use crate::McpClient;

/// Configuration for one stdio MCP server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

type Pending = Arc<Mutex<HashMap<u64, oneshot::Sender<Response>>>>;

pub struct StdioServer {
    name: String,
    stdin: Arc<Mutex<ChildStdin>>,
    pending: Pending,
    next_id: Arc<Mutex<u64>>,
    _child: Arc<Mutex<Child>>,
}

impl StdioServer {
    /// Spawn the MCP server process and start the reader loop.
    pub async fn spawn(name: impl Into<String>, config: StdioServerConfig) -> Result<Self> {
        let name = name.into();
        let mut cmd = Command::new(&config.command);
        cmd.args(&config.args);
        for (k, v) in &config.env {
            cmd.env(k, v);
        }
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let mut child = cmd
            .spawn()
            .with_context(|| format!("spawn mcp server '{}'", config.command))?;
        let stdin = child.stdin.take().ok_or_else(|| anyhow!("no stdin"))?;
        let stdout = child.stdout.take().ok_or_else(|| anyhow!("no stdout"))?;

        let pending: Pending = Arc::new(Mutex::new(HashMap::new()));
        let pending_reader = pending.clone();
        let reader_name = name.clone();

        // Reader task.
        tokio::spawn(async move {
            let mut lines = BufReader::new(stdout).lines();
            loop {
                match lines.next_line().await {
                    Ok(Some(line)) => {
                        if line.trim().is_empty() {
                            continue;
                        }
                        debug!(server = %reader_name, "<- {}", line);
                        let resp: Response = match serde_json::from_str(&line) {
                            Ok(r) => r,
                            Err(e) => {
                                warn!(server = %reader_name, line = %line, err = ?e, "bad json");
                                continue;
                            }
                        };
                        // Notifications have null id — skip.
                        let id = match resp.id.as_u64() {
                            Some(i) => i,
                            None => continue,
                        };
                        if let Some(tx) = pending_reader.lock().await.remove(&id) {
                            let _ = tx.send(resp);
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        warn!(server = %reader_name, err = ?e, "stdout read");
                        break;
                    }
                }
            }
        });

        // Drain stderr to debug logs (don't propagate as errors; some servers chat a lot).
        if let Some(stderr) = child.stderr.take() {
            let n = name.clone();
            tokio::spawn(async move {
                let mut lines = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    debug!(server = %n, "stderr: {}", line);
                }
            });
        }

        Ok(Self {
            name,
            stdin: Arc::new(Mutex::new(stdin)),
            pending,
            next_id: Arc::new(Mutex::new(1)),
            _child: Arc::new(Mutex::new(child)),
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
        let raw = serde_json::to_string(&req)? + "\n";

        let (tx, rx) = oneshot::channel();
        self.pending.lock().await.insert(id, tx);

        {
            let mut stdin = self.stdin.lock().await;
            stdin.write_all(raw.as_bytes()).await?;
            stdin.flush().await?;
        }

        let resp = tokio::time::timeout(std::time::Duration::from_secs(180), rx)
            .await
            .context("mcp request timed out")?
            .context("mcp pending dropped")?;

        if let Some(err) = resp.error {
            return Err(anyhow!("mcp error {}: {}", err.code, err.message));
        }
        Ok(resp.result.unwrap_or(serde_json::Value::Null))
    }

    async fn send_notification(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<()> {
        let n = Notification::new(method, params);
        let raw = serde_json::to_string(&n)? + "\n";
        let mut stdin = self.stdin.lock().await;
        stdin.write_all(raw.as_bytes()).await?;
        stdin.flush().await?;
        Ok(())
    }
}

#[async_trait]
impl McpClient for StdioServer {
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
        // Tell the server we're ready.
        self.send_notification("notifications/initialized", None)
            .await?;

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
        // We rely on kill_on_drop; nothing explicit needed.
        Ok(())
    }
}
