//! Built-in tools that the mimo agent can call.
//!
//! Each tool implements the [`Tool`] trait — name + JSON-Schema input + async
//! `run`. The agent loop in `mimo-tui-core` dispatches tool calls by name.
//!
//! Tools currently shipped:
//! - [`read_file`](mod@read_file): read a file (optional line range)
//! - [`write_file`](mod@write_file): create / overwrite a file
//! - [`edit_file`](mod@edit_file): replace specific text in an existing file
//! - [`shell`](mod@shell): run a shell command
//! - [`glob_tool`](mod@glob_tool): list files by glob pattern
//! - [`grep_tool`](mod@grep_tool): search file contents with regex
//! - [`web_fetch`](mod@web_fetch): fetch a URL (text only)
//! - [`todo`](mod@todo): manage a session-scoped task list

use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod edit_file;
pub mod glob_tool;
pub mod grep_tool;
pub mod read_file;
pub mod registry;
pub mod shell;
pub mod task;
pub mod todo;
pub mod web_fetch;
pub mod write_file;

pub use registry::{StaticRegistry, ToolRegistry};
pub use task::{TaskTool, ToolRegistryProvider};

/// Result of a tool invocation. Mirrors the Anthropic `tool_result` shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub output: String,
    pub is_error: bool,
}

impl ToolResult {
    pub fn ok(s: impl Into<String>) -> Self {
        Self {
            output: s.into(),
            is_error: false,
        }
    }

    pub fn err(s: impl Into<String>) -> Self {
        Self {
            output: s.into(),
            is_error: true,
        }
    }
}

/// Approval policy when a tool wants to do something risky (write a file, run shell).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalMode {
    /// Plan mode: deny all mutating operations.
    Plan,
    /// Agent mode: require user approval per call.
    Agent,
    /// Auto mode: don't require approval (YOLO).
    Auto,
}

/// Asks for user approval. The agent loop wires this up to a TUI prompt.
#[async_trait]
pub trait Approver: Send + Sync {
    /// Return `true` if the user approves running `tool_name` with `args`.
    async fn approve(&self, tool_name: &str, args: &serde_json::Value) -> bool;
}

/// Always-approve approver (for tests + Auto mode default).
pub struct AlwaysApprove;

#[async_trait]
impl Approver for AlwaysApprove {
    async fn approve(&self, _tool_name: &str, _args: &serde_json::Value) -> bool {
        true
    }
}

/// Always-deny approver (for Plan mode default).
pub struct AlwaysDeny;

#[async_trait]
impl Approver for AlwaysDeny {
    async fn approve(&self, _tool_name: &str, _args: &serde_json::Value) -> bool {
        false
    }
}

/// Context passed to a tool when it runs.
#[derive(Clone)]
pub struct ToolContext {
    /// Working directory for the tool (relative paths resolved against this).
    pub workspace: std::path::PathBuf,
    /// Approval mode + approver.
    pub mode: ApprovalMode,
    pub approver: Arc<dyn Approver>,
}

impl ToolContext {
    pub fn new(workspace: impl Into<std::path::PathBuf>) -> Self {
        Self {
            workspace: workspace.into(),
            mode: ApprovalMode::Agent,
            approver: Arc::new(AlwaysApprove),
        }
    }

    pub fn with_mode(mut self, mode: ApprovalMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_approver(mut self, approver: Arc<dyn Approver>) -> Self {
        self.approver = approver;
        self
    }

    /// Resolve a (possibly relative) path against the workspace.
    pub fn resolve(&self, path: impl AsRef<std::path::Path>) -> std::path::PathBuf {
        let p = path.as_ref();
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.workspace.join(p)
        }
    }
}

/// A tool the agent can invoke.
#[async_trait]
pub trait Tool: Send + Sync {
    /// Stable identifier (sent to the model).
    fn name(&self) -> &'static str;

    /// One-line description for the model.
    fn description(&self) -> &'static str;

    /// JSON Schema describing the input arguments.
    fn input_schema(&self) -> serde_json::Value;

    /// True if running this tool can mutate state and requires approval in Agent mode.
    fn is_destructive(&self) -> bool {
        false
    }

    /// Execute the tool.
    async fn run(&self, ctx: &ToolContext, input: serde_json::Value) -> anyhow::Result<ToolResult>;
}
