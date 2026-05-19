//! `write_file` — create a new file or overwrite an existing one.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use crate::{ApprovalMode, Tool, ToolContext, ToolResult};

#[derive(Deserialize)]
struct Input {
    path: String,
    content: String,
}

pub struct WriteFile;

#[async_trait]
impl Tool for WriteFile {
    fn name(&self) -> &'static str {
        "write_file"
    }

    fn description(&self) -> &'static str {
        "Create a new file or overwrite an existing one with the given content. \
         Use edit_file if you only want to change part of a file."
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path":    { "type": "string", "description": "File path" },
                "content": { "type": "string", "description": "Full file content" }
            },
            "required": ["path", "content"]
        })
    }

    fn is_destructive(&self) -> bool {
        true
    }

    async fn run(&self, ctx: &ToolContext, input: serde_json::Value) -> anyhow::Result<ToolResult> {
        if ctx.mode == ApprovalMode::Plan {
            return Ok(ToolResult::err("plan mode: write_file not permitted"));
        }
        if ctx.mode == ApprovalMode::Agent && !ctx.approver.approve("write_file", &input).await {
            return Ok(ToolResult::err("user denied write_file"));
        }

        let Input { path, content } = serde_json::from_value(input)?;
        let abs = ctx.resolve(&path);

        if let Some(parent) = abs.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        match tokio::fs::write(&abs, content.as_bytes()).await {
            Ok(()) => Ok(ToolResult::ok(format!(
                "wrote {} bytes to {}",
                content.len(),
                path
            ))),
            Err(e) => Ok(ToolResult::err(format!("write {}: {}", path, e))),
        }
    }
}
