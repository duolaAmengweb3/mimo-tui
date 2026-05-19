//! Agent main loop.
//!
//! Each turn:
//! 1. Append the user's message to the conversation
//! 2. POST /v1/messages with tools enabled
//! 3. For each tool_use block returned, invoke the matching tool, get a
//!    tool_result, post back, loop until the model stops requesting tools
//! 4. Yield the final assistant text

use std::sync::Arc;

use anyhow::Result;
use futures::StreamExt;
use mimo_tui_anthropic_client::{
    stream, BlockDelta, Client, ContentBlock, Message, MessageContent, MessagesRequest, Role,
    StopReason, StreamEvent, SystemPrompt, ToolResultContent,
};
use mimo_tui_skills::SkillRegistry;
use mimo_tui_tools::{ApprovalMode, ToolContext, ToolRegistry};
use tokio::sync::mpsc;
use tracing::debug;

use crate::config::{AgentModeConfig, Config, Language};
use crate::session::Session;
use crate::usage::UsageDb;

#[derive(Debug, Clone)]
pub enum AgentEvent {
    /// Streaming text delta (one chunk; concatenate to get the full reply).
    TextDelta(String),
    /// Streaming thinking delta.
    ThinkingDelta(String),
    /// Final assistant text for this turn (concatenation of all text blocks).
    AssistantText(String),
    /// A complete thinking block (also emitted incrementally as ThinkingDelta).
    Thinking(String),
    /// A tool was invoked.
    ToolCall {
        name: String,
        args: serde_json::Value,
        result: String,
        is_error: bool,
    },
    /// Token usage for the latest API call.
    Usage(mimo_tui_anthropic_client::Usage),
    /// Non-fatal error.
    Error(String),
}

#[derive(Debug, Clone)]
pub struct AgentReply {
    pub text: String,
    pub events: Vec<AgentEvent>,
}

pub struct Agent {
    pub client: Client,
    pub config: Config,
    pub tools: Arc<ToolRegistry>,
    pub ctx: ToolContext,
    pub session: Session,
    pub usage_db: Option<UsageDb>,
    pub skills: Arc<SkillRegistry>,
}

impl Agent {
    pub fn new(
        client: Client,
        config: Config,
        tools: Arc<ToolRegistry>,
        ctx: ToolContext,
        session: Session,
    ) -> Self {
        Self {
            client,
            config,
            tools,
            ctx,
            session,
            usage_db: UsageDb::open().ok(),
            skills: Arc::new(SkillRegistry::new()),
        }
    }

    pub fn with_skills(mut self, skills: Arc<SkillRegistry>) -> Self {
        self.skills = skills;
        self
    }

    /// Run one user turn. Streams events through `tx` and returns the final reply.
    pub async fn run_turn(
        &mut self,
        user_input: &str,
        tx: mpsc::UnboundedSender<AgentEvent>,
    ) -> Result<AgentReply> {
        self.session.messages.push(Message::user_text(user_input));
        let mut all_events = Vec::new();
        let mut final_text = String::new();

        for iteration in 0..self.config.max_iterations {
            debug!(iter = iteration, "agent iteration");

            let mut req = MessagesRequest::new(&self.config.model, self.config.max_tokens);
            req.messages = self.session.messages.clone();
            // Inject relevant skills on the first iteration based on user input.
            let mut sys = system_prompt(self.config.language);
            if iteration == 0 {
                let matched = self.skills.select_for(user_input);
                if !matched.is_empty() {
                    sys.push_str("\n\n");
                    for s in matched {
                        sys.push_str(&s.render());
                        sys.push('\n');
                    }
                }
            }
            req.system = Some(SystemPrompt::Text(sys));
            req.tools = Some(serde_json::from_value(serde_json::to_value(
                self.tools.as_anthropic_tools(),
            )?)?);

            // Streaming: start the request, decode SSE events live.
            let raw = match self.client.messages_stream_raw(req).await {
                Ok(r) => r,
                Err(e) => {
                    let msg = format!("API error: {}", e);
                    let _ = tx.send(AgentEvent::Error(msg.clone()));
                    all_events.push(AgentEvent::Error(msg.clone()));
                    return Ok(AgentReply {
                        text: msg,
                        events: all_events,
                    });
                }
            };

            let resp = match collect_streaming(raw, &tx, &mut all_events).await {
                Ok(r) => r,
                Err(e) => {
                    let msg = format!("stream error: {}", e);
                    let _ = tx.send(AgentEvent::Error(msg.clone()));
                    all_events.push(AgentEvent::Error(msg.clone()));
                    return Ok(AgentReply {
                        text: msg,
                        events: all_events,
                    });
                }
            };

            if let Some(db) = &self.usage_db {
                let _ = db.record(&resp.model, &resp.usage);
            }
            let usage_event = AgentEvent::Usage(resp.usage.clone());
            let _ = tx.send(usage_event.clone());
            all_events.push(usage_event);

            // Capture text blocks for final reply.
            for block in &resp.content {
                if let ContentBlock::Text { text, .. } = block {
                    final_text.push_str(text);
                }
            }

            // Push the model's assistant message into history.
            self.session
                .messages
                .push(Message::assistant_blocks(resp.content.clone()));

            // If we're done, return.
            let want_tools = matches!(resp.stop_reason, Some(StopReason::ToolUse));
            if !want_tools {
                let _ = self.session.save();
                return Ok(AgentReply {
                    text: final_text.clone(),
                    events: all_events,
                });
            }

            // Otherwise: execute every tool_use block and append tool_result.
            let mut results: Vec<ContentBlock> = Vec::new();
            for block in &resp.content {
                if let ContentBlock::ToolUse { id, name, input } = block {
                    let tool = match self.tools.get(name) {
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

                    let result = tool.run(&self.ctx, input.clone()).await;
                    let (out_text, is_err) = match result {
                        Ok(r) => (r.output, r.is_error),
                        Err(e) => (format!("tool panicked: {}", e), true),
                    };
                    let ev = AgentEvent::ToolCall {
                        name: name.clone(),
                        args: input.clone(),
                        result: out_text.clone(),
                        is_error: is_err,
                    };
                    let _ = tx.send(ev.clone());
                    all_events.push(ev);

                    results.push(ContentBlock::ToolResult {
                        tool_use_id: id.clone(),
                        content: ToolResultContent::Text(out_text),
                        is_error: is_err,
                    });
                }
            }

            // Append the tool_result(s) as a single user message.
            self.session.messages.push(Message {
                role: Role::User,
                content: MessageContent::Blocks(results),
            });
        }

        let msg = format!("max iterations ({}) reached", self.config.max_iterations);
        let _ = tx.send(AgentEvent::Error(msg.clone()));
        all_events.push(AgentEvent::Error(msg.clone()));
        let _ = self.session.save();
        Ok(AgentReply {
            text: final_text,
            events: all_events,
        })
    }
}

/// Decode the streaming response, emitting TextDelta / ThinkingDelta events
/// in real time, and return the assembled `MessagesResponse` when complete.
async fn collect_streaming(
    raw: reqwest::Response,
    tx: &mpsc::UnboundedSender<AgentEvent>,
    all_events: &mut Vec<AgentEvent>,
) -> Result<mimo_tui_anthropic_client::MessagesResponse> {
    use mimo_tui_anthropic_client::MessagesResponse;

    let mut events_stream = Box::pin(stream::events(raw));
    let mut response: Option<MessagesResponse> = None;
    let mut blocks: Vec<ContentBlock> = Vec::new();
    let mut text_buf: Vec<String> = Vec::new();
    let mut thinking_buf: Vec<String> = Vec::new();
    let mut input_buf: Vec<String> = Vec::new();
    let mut sig_buf: Vec<String> = Vec::new();

    while let Some(item) = events_stream.next().await {
        let ev = item?;
        match ev {
            StreamEvent::MessageStart { message } => response = Some(message),
            StreamEvent::ContentBlockStart {
                index,
                content_block,
            } => {
                let i = index as usize;
                ensure_capacity(
                    &mut blocks,
                    &mut text_buf,
                    &mut thinking_buf,
                    &mut input_buf,
                    &mut sig_buf,
                    i + 1,
                );
                blocks[i] = content_block;
            }
            StreamEvent::ContentBlockDelta { index, delta } => {
                let i = index as usize;
                ensure_capacity(
                    &mut blocks,
                    &mut text_buf,
                    &mut thinking_buf,
                    &mut input_buf,
                    &mut sig_buf,
                    i + 1,
                );
                match delta {
                    BlockDelta::TextDelta { text } => {
                        text_buf[i].push_str(&text);
                        let evt = AgentEvent::TextDelta(text);
                        let _ = tx.send(evt.clone());
                        all_events.push(evt);
                    }
                    BlockDelta::ThinkingDelta { thinking } => {
                        thinking_buf[i].push_str(&thinking);
                        let evt = AgentEvent::ThinkingDelta(thinking);
                        let _ = tx.send(evt.clone());
                        all_events.push(evt);
                    }
                    BlockDelta::InputJsonDelta { partial_json } => {
                        input_buf[i].push_str(&partial_json)
                    }
                    BlockDelta::SignatureDelta { signature } => sig_buf[i].push_str(&signature),
                }
            }
            StreamEvent::ContentBlockStop { index } => {
                let i = index as usize;
                if let Some(block) = blocks.get_mut(i) {
                    match block {
                        ContentBlock::Text { text, .. } => text.push_str(&text_buf[i]),
                        ContentBlock::Thinking {
                            thinking,
                            signature,
                        } => {
                            thinking.push_str(&thinking_buf[i]);
                            if !sig_buf[i].is_empty() {
                                *signature = sig_buf[i].clone();
                            }
                        }
                        ContentBlock::ToolUse { input, name, .. } if !input_buf[i].is_empty() => {
                            match serde_json::from_str::<serde_json::Value>(&input_buf[i]) {
                                Ok(v) => *input = v,
                                Err(e) => {
                                    tracing::warn!(
                                        tool = %name,
                                        error = ?e,
                                        partial_json = %input_buf[i],
                                        "tool_use input did not parse as JSON; using empty input"
                                    );
                                    let _ = tx.send(AgentEvent::Error(format!(
                                        "tool_use[{}] input JSON malformed: {}",
                                        name, e
                                    )));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            StreamEvent::MessageDelta { delta, usage } => {
                if let Some(r) = response.as_mut() {
                    if delta.stop_reason.is_some() {
                        r.stop_reason = delta.stop_reason;
                    }
                    if delta.stop_sequence.is_some() {
                        r.stop_sequence = delta.stop_sequence;
                    }
                    r.usage.output_tokens = usage.output_tokens;
                    if usage.cache_read_input_tokens > 0 {
                        r.usage.cache_read_input_tokens = usage.cache_read_input_tokens;
                    }
                    if usage.cache_creation_input_tokens > 0 {
                        r.usage.cache_creation_input_tokens = usage.cache_creation_input_tokens;
                    }
                }
            }
            StreamEvent::MessageStop => break,
            StreamEvent::Ping => {}
            StreamEvent::Error { error } => {
                return Err(anyhow::anyhow!("server stream error: {}", error));
            }
        }
    }

    let mut r = response.unwrap_or(MessagesResponse {
        id: String::new(),
        kind: "message".to_string(),
        role: Role::Assistant,
        model: String::new(),
        content: Vec::new(),
        stop_reason: None,
        stop_sequence: None,
        usage: Default::default(),
    });
    r.content = blocks;
    Ok(r)
}

fn ensure_capacity(
    blocks: &mut Vec<ContentBlock>,
    text_buf: &mut Vec<String>,
    thinking_buf: &mut Vec<String>,
    input_buf: &mut Vec<String>,
    sig_buf: &mut Vec<String>,
    needed: usize,
) {
    while blocks.len() < needed {
        blocks.push(ContentBlock::Text {
            text: String::new(),
            cache_control: None,
        });
        text_buf.push(String::new());
        thinking_buf.push(String::new());
        input_buf.push(String::new());
        sig_buf.push(String::new());
    }
}

/// Maps the user's mode config to a ToolContext mode.
pub fn approval_mode(cfg: AgentModeConfig) -> ApprovalMode {
    match cfg {
        AgentModeConfig::Plan => ApprovalMode::Plan,
        AgentModeConfig::Agent => ApprovalMode::Agent,
        AgentModeConfig::Auto => ApprovalMode::Auto,
    }
}

fn system_prompt(lang: Language) -> String {
    match lang {
        Language::Zh => {
            r#"你是 mimo-tui 的 AI 编程助手，跑在用户的终端里。底层是小米 MiMo 模型。

工作方式：
- 用户给你一个任务，你用提供的工具读代码、改代码、跑命令完成它
- 重要操作（写文件、跑 shell）需要用户审批，遵守审批结果
- 任务复杂时，先用 todo 工具列计划再动手
- 遇到不确定的事，告诉用户并问清楚，不要瞎猜

每一步都要简洁。完成任务后用一两句话总结。"#
                .to_string()
        }
        Language::En => {
            r#"You are mimo-tui's AI coding assistant, running in the user's terminal. Powered by Xiaomi's MiMo model.

How you work:
- The user gives you a task; you use the provided tools to read code, edit files, run commands
- Destructive operations (writing files, shell commands) may require approval — respect it
- For complex tasks, plan first using the todo tool before doing
- When uncertain, ask the user rather than guessing

Be concise at each step. Summarize in one or two sentences when done."#
                .to_string()
        }
    }
}
