//! `read_file` — read a file's contents, optionally a line range.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use crate::{Tool, ToolContext, ToolResult};

#[derive(Deserialize)]
struct Input {
    path: String,
    #[serde(default)]
    start_line: Option<usize>,
    #[serde(default)]
    end_line: Option<usize>,
    /// Defaults to 2000 to mirror Claude Code behaviour.
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    2000
}

pub struct ReadFile;

#[async_trait]
impl Tool for ReadFile {
    fn name(&self) -> &'static str {
        "read_file"
    }

    fn description(&self) -> &'static str {
        "Read a text file from disk. Returns the content with line numbers. \
         Use start_line/end_line to read a slice of a large file."
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path":       { "type": "string", "description": "File path (relative paths resolve to workspace)" },
                "start_line": { "type": "integer", "description": "1-based start line (inclusive)" },
                "end_line":   { "type": "integer", "description": "1-based end line (inclusive)" },
                "limit":      { "type": "integer", "description": "Max number of lines to return (default 2000)" }
            },
            "required": ["path"]
        })
    }

    async fn run(&self, ctx: &ToolContext, input: serde_json::Value) -> anyhow::Result<ToolResult> {
        let Input {
            path,
            start_line,
            end_line,
            limit,
        } = serde_json::from_value(input)?;

        let abs = ctx.resolve(&path);
        let raw = match tokio::fs::read_to_string(&abs).await {
            Ok(s) => s,
            Err(e) => return Ok(ToolResult::err(format!("read {}: {}", path, e))),
        };

        let lines: Vec<&str> = raw.lines().collect();
        let total = lines.len();

        let start = start_line.unwrap_or(1).max(1).min(total.max(1));
        let end = end_line.unwrap_or(total).min(total);
        let end = end.min(start + limit - 1);

        if start > end {
            return Ok(ToolResult::ok(format!("(empty range) total {} lines", total)));
        }

        let mut out = String::with_capacity(raw.len());
        for (idx, line) in lines.iter().enumerate().take(end).skip(start.saturating_sub(1)) {
            use std::fmt::Write as _;
            let _ = writeln!(out, "{:>5}\t{}", idx + 1, line);
        }
        if end < total {
            use std::fmt::Write as _;
            let _ = writeln!(out, "... ({} more lines, total {})", total - end, total);
        }

        Ok(ToolResult::ok(out))
    }
}
