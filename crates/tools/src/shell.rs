//! `shell` — run a shell command in the workspace.

use std::time::Duration;

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use tokio::time::timeout;

use crate::{ApprovalMode, Tool, ToolContext, ToolResult};

#[derive(Deserialize)]
struct Input {
    command: String,
    #[serde(default = "default_timeout")]
    timeout_secs: u64,
}

fn default_timeout() -> u64 {
    120
}

const MAX_OUTPUT_BYTES: usize = 64 * 1024;

pub struct Shell;

#[async_trait]
impl Tool for Shell {
    fn name(&self) -> &'static str {
        "shell"
    }

    fn description(&self) -> &'static str {
        "Run a shell command in the workspace. Returns combined stdout+stderr. \
         Output is truncated to 64KB. Use for tests, git, build commands, etc."
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "command":      { "type": "string", "description": "Command to run via /bin/sh -c" },
                "timeout_secs": { "type": "integer", "default": 120 }
            },
            "required": ["command"]
        })
    }

    fn is_destructive(&self) -> bool {
        true
    }

    async fn run(&self, ctx: &ToolContext, input: serde_json::Value) -> anyhow::Result<ToolResult> {
        if ctx.mode == ApprovalMode::Plan {
            return Ok(ToolResult::err("plan mode: shell not permitted"));
        }
        if ctx.mode == ApprovalMode::Agent && !ctx.approver.approve("shell", &input).await {
            return Ok(ToolResult::err("user denied shell"));
        }

        let Input { command, timeout_secs } = serde_json::from_value(input)?;
        let mut cmd = tokio::process::Command::new("/bin/sh");
        cmd.arg("-c").arg(&command).current_dir(&ctx.workspace);

        let fut = cmd.output();
        let output = match timeout(Duration::from_secs(timeout_secs), fut).await {
            Ok(Ok(o)) => o,
            Ok(Err(e)) => return Ok(ToolResult::err(format!("spawn failed: {}", e))),
            Err(_) => return Ok(ToolResult::err(format!("command timed out after {}s", timeout_secs))),
        };

        let mut combined = Vec::with_capacity(output.stdout.len() + output.stderr.len());
        combined.extend_from_slice(&output.stdout);
        if !output.stderr.is_empty() {
            combined.extend_from_slice(b"\n--- stderr ---\n");
            combined.extend_from_slice(&output.stderr);
        }
        let truncated = combined.len() > MAX_OUTPUT_BYTES;
        if truncated {
            combined.truncate(MAX_OUTPUT_BYTES);
        }
        let mut body = String::from_utf8_lossy(&combined).to_string();
        if truncated {
            body.push_str("\n\n... (output truncated to 64KB)");
        }

        let result = if output.status.success() {
            ToolResult::ok(body)
        } else {
            ToolResult::err(format!(
                "exit code {}\n{}",
                output.status.code().unwrap_or(-1),
                body
            ))
        };
        Ok(result)
    }
}
