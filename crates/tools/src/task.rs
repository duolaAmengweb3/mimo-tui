//! `task` — dispatch a sub-agent to run a focused mini-task in parallel.
//!
//! The parent agent calls `task` with a `description` (free-form natural
//! language) plus optional `model` override. We spin up a sub-agent against
//! the same client + tool registry, with a system prompt locked to "you are
//! a sub-agent — finish the task in one or two tool turns and return a short
//! result". Up to 10 sub-agents may run in parallel; further calls block.
//!
//! The result string returned to the parent is the sub-agent's final
//! assistant text.

use std::sync::Arc;

use async_trait::async_trait;
use mimo_tui_anthropic_client::{
    Client, ContentBlock, Message, MessagesRequest, StopReason, SystemPrompt, ToolResultContent,
};
use serde::Deserialize;
use serde_json::json;
use tokio::sync::Semaphore;

use crate::{Tool, ToolContext, ToolResult};

const SUBAGENT_SLOTS: usize = 10;

#[derive(Deserialize)]
struct Input {
    description: String,
    #[serde(default)]
    model: Option<String>,
    #[serde(default = "default_max_tokens")]
    max_tokens: u32,
    #[serde(default = "default_max_iters")]
    max_iterations: u32,
}

fn default_max_tokens() -> u32 {
    2048
}

fn default_max_iters() -> u32 {
    8
}

/// Sub-agent dispatcher.
///
/// Holds the same Anthropic client + tool registry the parent uses so the
/// sub-agent can read files, run shell, etc. Maintains a semaphore that
/// caps concurrent sub-agents at [`SUBAGENT_SLOTS`].
pub struct TaskTool {
    client: Client,
    tools: Arc<dyn ToolRegistryProvider>,
    /// Model the sub-agent defaults to (parent's model unless overridden).
    default_model: String,
    semaphore: Arc<Semaphore>,
}

/// Read-only access to the parent registry so the sub-agent can see the same
/// tools without we storing a hard reference to ToolRegistry (which would be
/// recursive — the task tool *is* in the registry).
pub trait ToolRegistryProvider: Send + Sync {
    fn tools(&self) -> Vec<Arc<dyn Tool>>;
}

impl TaskTool {
    pub fn new(client: Client, tools: Arc<dyn ToolRegistryProvider>, default_model: impl Into<String>) -> Self {
        Self {
            client,
            tools,
            default_model: default_model.into(),
            semaphore: Arc::new(Semaphore::new(SUBAGENT_SLOTS)),
        }
    }
}

#[async_trait]
impl Tool for TaskTool {
    fn name(&self) -> &'static str {
        "task"
    }

    fn description(&self) -> &'static str {
        "Dispatch a focused sub-agent. Use this to parallelise independent \
         pieces of work (file analysis, multi-file search, codebase Q&A). \
         The sub-agent returns one final string."
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "description":    { "type": "string", "description": "What the sub-agent should do (be specific)" },
                "model":          { "type": "string", "description": "Override model (default: parent's)" },
                "max_tokens":     { "type": "integer", "default": 2048 },
                "max_iterations": { "type": "integer", "default": 8 }
            },
            "required": ["description"]
        })
    }

    async fn run(&self, ctx: &ToolContext, input: serde_json::Value) -> anyhow::Result<ToolResult> {
        let Input {
            description,
            model,
            max_tokens,
            max_iterations,
        } = serde_json::from_value(input)?;

        let permit = self.semaphore.clone().acquire_owned().await?;
        let model = model.unwrap_or_else(|| self.default_model.clone());
        let tools = self.tools.tools();

        let result = run_subagent(
            &self.client,
            &tools,
            ctx,
            &description,
            &model,
            max_tokens,
            max_iterations,
        )
        .await;
        drop(permit);

        match result {
            Ok(text) => Ok(ToolResult::ok(text)),
            Err(e) => Ok(ToolResult::err(format!("sub-agent failed: {}", e))),
        }
    }
}

async fn run_subagent(
    client: &Client,
    parent_tools: &[Arc<dyn Tool>],
    ctx: &ToolContext,
    description: &str,
    model: &str,
    max_tokens: u32,
    max_iterations: u32,
) -> anyhow::Result<String> {
    // Build tool list, excluding `task` itself to prevent recursion.
    let tool_defs: Vec<serde_json::Value> = parent_tools
        .iter()
        .filter(|t| t.name() != "task")
        .map(|t| {
            json!({
                "name": t.name(),
                "description": t.description(),
                "input_schema": t.input_schema(),
            })
        })
        .collect();
    let tools_value: Vec<mimo_tui_anthropic_client::Tool> = serde_json::from_value(serde_json::to_value(tool_defs)?)?;

    let mut messages: Vec<Message> = vec![Message::user_text(description)];
    let system = SystemPrompt::Text(SUBAGENT_SYSTEM_PROMPT.to_string());

    for _ in 0..max_iterations {
        let mut req = MessagesRequest::new(model, max_tokens);
        req.messages = messages.clone();
        req.system = Some(system.clone());
        req.tools = Some(tools_value.clone());

        let resp = client.messages(req).await?;

        // Collect final text.
        let want_tools = matches!(resp.stop_reason, Some(StopReason::ToolUse));
        if !want_tools {
            let text = resp
                .content
                .iter()
                .filter_map(|b| match b {
                    ContentBlock::Text { text, .. } => Some(text.as_str()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join("");
            return Ok(text);
        }

        messages.push(Message::assistant_blocks(resp.content.clone()));

        let mut results: Vec<ContentBlock> = Vec::new();
        for block in &resp.content {
            if let ContentBlock::ToolUse { id, name, input } = block {
                let tool = match parent_tools.iter().find(|t| t.name() == name) {
                    Some(t) => t.clone(),
                    None => {
                        results.push(ContentBlock::ToolResult {
                            tool_use_id: id.clone(),
                            content: ToolResultContent::Text(format!("unknown tool: {}", name)),
                            is_error: true,
                        });
                        continue;
                    }
                };
                let out = tool.run(ctx, input.clone()).await;
                let (text, is_err) = match out {
                    Ok(r) => (r.output, r.is_error),
                    Err(e) => (format!("tool error: {}", e), true),
                };
                results.push(ContentBlock::ToolResult {
                    tool_use_id: id.clone(),
                    content: ToolResultContent::Text(text),
                    is_error: is_err,
                });
            }
        }
        messages.push(Message {
            role: mimo_tui_anthropic_client::Role::User,
            content: mimo_tui_anthropic_client::MessageContent::Blocks(results),
        });
    }

    Err(anyhow::anyhow!("sub-agent hit max_iterations {}", max_iterations))
}

const SUBAGENT_SYSTEM_PROMPT: &str = r#"你是一个 mimo-tui 子 agent。你被父 agent 派来做一个聚焦的、可独立完成的小任务。

工作原则：
- 用提供的工具完成任务（read_file / shell / grep / write_file 等）
- 一两轮工具调用就该出结果
- 不要把任务丢回给父 agent，自己干完
- 完成后回一段简洁的最终结果（不要 markdown 标题、不要长篇大论，直接给父 agent 能读的纯文本结论）"#;
