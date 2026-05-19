//! `todo` — a session-scoped task list the agent can read / write.

use std::sync::Mutex;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{Tool, ToolContext, ToolResult};

#[derive(Clone, Default)]
pub struct TodoList {
    items: std::sync::Arc<Mutex<Vec<TodoItem>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u32,
    pub text: String,
    pub status: TodoStatus,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Done,
}

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
enum Input {
    Add { text: String },
    Update { id: u32, status: TodoStatus },
    List,
    Clear,
}

pub struct Todo {
    pub list: TodoList,
}

impl Todo {
    pub fn new(list: TodoList) -> Self {
        Self { list }
    }
}

#[async_trait]
impl Tool for Todo {
    fn name(&self) -> &'static str {
        "todo"
    }

    fn description(&self) -> &'static str {
        "Manage a session todo list. Use this to plan multi-step work. \
         Actions: add | update | list | clear."
    }

    fn input_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": { "type": "string", "enum": ["add", "update", "list", "clear"] },
                "text":   { "type": "string" },
                "id":     { "type": "integer" },
                "status": { "type": "string", "enum": ["pending", "in_progress", "done"] }
            },
            "required": ["action"]
        })
    }

    async fn run(
        &self,
        _ctx: &ToolContext,
        input: serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        let action: Input = serde_json::from_value(input)?;
        let mut items = self.list.items.lock().unwrap();
        match action {
            Input::Add { text } => {
                let id = items.iter().map(|i| i.id).max().unwrap_or(0) + 1;
                items.push(TodoItem {
                    id,
                    text,
                    status: TodoStatus::Pending,
                });
            }
            Input::Update { id, status } => {
                if let Some(it) = items.iter_mut().find(|i| i.id == id) {
                    it.status = status;
                } else {
                    return Ok(ToolResult::err(format!("no todo with id {}", id)));
                }
            }
            Input::Clear => items.clear(),
            Input::List => {}
        };

        let mut out = String::new();
        for i in items.iter() {
            use std::fmt::Write as _;
            let icon = match i.status {
                TodoStatus::Pending => "[ ]",
                TodoStatus::InProgress => "[~]",
                TodoStatus::Done => "[x]",
            };
            let _ = writeln!(out, "{} #{} {}", icon, i.id, i.text);
        }
        if out.is_empty() {
            out = "(no todos)".to_string();
        }
        Ok(ToolResult::ok(out))
    }
}
