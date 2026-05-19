//! `grep` — search file contents with a regex.

use std::io::{BufRead, BufReader};

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use crate::{Tool, ToolContext, ToolResult};

#[derive(Deserialize)]
struct Input {
    pattern: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default = "default_limit")]
    limit: usize,
    #[serde(default)]
    case_sensitive: bool,
}

fn default_limit() -> usize {
    100
}

pub struct GrepTool;

#[async_trait]
impl Tool for GrepTool {
    fn name(&self) -> &'static str {
        "grep"
    }

    fn description(&self) -> &'static str {
        "Search file contents in the workspace with a regex. \
         Returns file:line:matched-text. Respects .gitignore."
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "pattern":        { "type": "string", "description": "Regex pattern" },
                "path":           { "type": "string", "description": "Optional sub-path to limit search" },
                "limit":          { "type": "integer", "default": 100 },
                "case_sensitive": { "type": "boolean", "default": false }
            },
            "required": ["pattern"]
        })
    }

    async fn run(&self, ctx: &ToolContext, input: serde_json::Value) -> anyhow::Result<ToolResult> {
        let Input {
            pattern,
            path,
            limit,
            case_sensitive,
        } = serde_json::from_value(input)?;

        let re_pattern = if case_sensitive {
            pattern.clone()
        } else {
            format!("(?i){}", pattern)
        };
        let re = match regex::Regex::new(&re_pattern) {
            Ok(r) => r,
            Err(e) => return Ok(ToolResult::err(format!("bad regex '{}': {}", pattern, e))),
        };

        let root = match path {
            Some(p) => ctx.resolve(p),
            None => ctx.workspace.clone(),
        };

        let walker = ignore::WalkBuilder::new(&root)
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
            let file_path = entry.path();
            let display_path = file_path
                .strip_prefix(&ctx.workspace)
                .unwrap_or(file_path)
                .display()
                .to_string();

            let f = match std::fs::File::open(file_path) {
                Ok(f) => f,
                Err(_) => continue,
            };
            for (line_no, line) in BufReader::new(f).lines().enumerate() {
                let line = match line {
                    Ok(l) => l,
                    Err(_) => break, // probably binary
                };
                if re.is_match(&line) {
                    let truncated: String = line.chars().take(200).collect();
                    hits.push(format!("{}:{}: {}", display_path, line_no + 1, truncated));
                    if hits.len() >= limit {
                        break;
                    }
                }
            }
            if hits.len() >= limit {
                break;
            }
        }

        if hits.is_empty() {
            Ok(ToolResult::ok(format!("(no matches for {})", pattern)))
        } else {
            Ok(ToolResult::ok(hits.join("\n")))
        }
    }
}
