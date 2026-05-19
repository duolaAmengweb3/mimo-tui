//! Tool registry.
//!
//! Wires up the default tool set and exposes name-based lookup.

use std::collections::HashMap;
use std::sync::Arc;

use crate::Tool;
use crate::{
    edit_file::EditFile, glob_tool::GlobTool, grep_tool::GrepTool, read_file::ReadFile,
    shell::Shell, todo::Todo, web_fetch::WebFetch, write_file::WriteFile,
};

pub struct ToolRegistry {
    tools: HashMap<&'static str, Arc<dyn Tool>>,
}

impl ToolRegistry {
    /// Empty registry.
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Pre-loaded with all built-in tools.
    pub fn with_defaults() -> Self {
        let mut r = Self::new();
        r.insert(Arc::new(ReadFile));
        r.insert(Arc::new(WriteFile));
        r.insert(Arc::new(EditFile));
        r.insert(Arc::new(Shell));
        r.insert(Arc::new(GlobTool));
        r.insert(Arc::new(GrepTool));
        r.insert(Arc::new(WebFetch));
        r.insert(Arc::new(Todo::new(crate::todo::TodoList::default())));
        r
    }

    pub fn insert(&mut self, tool: Arc<dyn Tool>) -> &mut Self {
        self.tools.insert(tool.name(), tool);
        self
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.get(name)
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.tools.keys().copied().collect()
    }

    /// Render all tools as Anthropic-style tool definitions (for the request).
    pub fn as_anthropic_tools(&self) -> Vec<serde_json::Value> {
        self.tools
            .values()
            .map(|t| {
                serde_json::json!({
                    "name":        t.name(),
                    "description": t.description(),
                    "input_schema": t.input_schema(),
                })
            })
            .collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}
