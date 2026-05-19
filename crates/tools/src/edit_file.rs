//! `edit_file` — replace an exact text fragment in an existing file.
//!
//! Tries to match Claude Code's "string replacement" semantics: `old_string`
//! must be unique in the file (otherwise we refuse and ask for more context),
//! and the replacement is applied once.

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;

use crate::{ApprovalMode, Tool, ToolContext, ToolResult};

#[derive(Deserialize)]
struct Input {
    path: String,
    old_string: String,
    new_string: String,
    #[serde(default)]
    replace_all: bool,
}

pub struct EditFile;

#[async_trait]
impl Tool for EditFile {
    fn name(&self) -> &'static str {
        "edit_file"
    }

    fn description(&self) -> &'static str {
        "Replace exact text in an existing file. old_string must match exactly \
         and be unique unless replace_all=true."
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path":        { "type": "string" },
                "old_string":  { "type": "string", "description": "Exact text to replace" },
                "new_string":  { "type": "string", "description": "Replacement text" },
                "replace_all": { "type": "boolean", "default": false }
            },
            "required": ["path", "old_string", "new_string"]
        })
    }

    fn is_destructive(&self) -> bool {
        true
    }

    async fn run(&self, ctx: &ToolContext, input: serde_json::Value) -> anyhow::Result<ToolResult> {
        if ctx.mode == ApprovalMode::Plan {
            return Ok(ToolResult::err("plan mode: edit_file not permitted"));
        }
        if ctx.mode == ApprovalMode::Agent && !ctx.approver.approve("edit_file", &input).await {
            return Ok(ToolResult::err("user denied edit_file"));
        }

        let Input {
            path,
            old_string,
            new_string,
            replace_all,
        } = serde_json::from_value(input)?;

        let abs = ctx.resolve(&path);
        let original = match tokio::fs::read_to_string(&abs).await {
            Ok(s) => s,
            Err(e) => return Ok(ToolResult::err(format!("read {}: {}", path, e))),
        };

        let occurrences = original.matches(&old_string).count();
        if occurrences == 0 {
            return Ok(ToolResult::err(format!(
                "old_string not found in {} — must match exactly (whitespace counts)",
                path
            )));
        }
        if occurrences > 1 && !replace_all {
            return Ok(ToolResult::err(format!(
                "old_string matches {} times in {}; add more context or set replace_all=true",
                occurrences, path
            )));
        }

        let new_content = if replace_all {
            original.replace(&old_string, &new_string)
        } else {
            original.replacen(&old_string, &new_string, 1)
        };

        tokio::fs::write(&abs, new_content.as_bytes()).await?;
        Ok(ToolResult::ok(format!(
            "edited {} ({} replacement{})",
            path,
            occurrences.min(if replace_all { occurrences } else { 1 }),
            if occurrences > 1 && replace_all { "s" } else { "" }
        )))
    }
}
