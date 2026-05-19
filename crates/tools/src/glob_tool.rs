//! `glob` — list files matching a glob pattern.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use crate::{Tool, ToolContext, ToolResult};

#[derive(Deserialize)]
struct Input {
    pattern: String,
    #[serde(default = "default_limit")]
    limit: usize,
}

fn default_limit() -> usize {
    100
}

pub struct GlobTool;

#[async_trait]
impl Tool for GlobTool {
    fn name(&self) -> &'static str {
        "glob"
    }

    fn description(&self) -> &'static str {
        "List files in the workspace matching a glob pattern (e.g. **/*.rs). \
         Respects .gitignore."
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "pattern": { "type": "string", "description": "Glob pattern" },
                "limit":   { "type": "integer", "default": 100 }
            },
            "required": ["pattern"]
        })
    }

    async fn run(&self, ctx: &ToolContext, input: serde_json::Value) -> anyhow::Result<ToolResult> {
        let Input { pattern, limit } = serde_json::from_value(input)?;

        let mut override_builder = ignore::overrides::OverrideBuilder::new(&ctx.workspace);
        if let Err(e) = override_builder.add(&pattern) {
            return Ok(ToolResult::err(format!("bad pattern '{}': {}", pattern, e)));
        }
        let overrides = override_builder.build()?;

        let walker = ignore::WalkBuilder::new(&ctx.workspace)
            .overrides(overrides)
            .standard_filters(true)
            .build();

        let mut hits: Vec<String> = Vec::new();
        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };
            if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                continue;
            }
            let path = entry.path();
            if let Ok(rel) = path.strip_prefix(&ctx.workspace) {
                hits.push(rel.display().to_string());
            }
            if hits.len() >= limit {
                break;
            }
        }

        if hits.is_empty() {
            Ok(ToolResult::ok(format!("(no files match {})", pattern)))
        } else {
            Ok(ToolResult::ok(hits.join("\n")))
        }
    }
}
