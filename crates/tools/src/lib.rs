//! 12 个工具实现
//!
//! 工具列表（待实现）：
//! - `read_file`   读文件（含行号范围）
//! - `write_file`  写新文件
//! - `edit_file`   diff 修改
//! - `apply_patch` 应用 patch（兼容 Claude Code 格式）
//! - `shell`       shell 执行（含审批）
//! - `glob`        文件匹配
//! - `grep`        内容搜索（用 ripgrep crate）
//! - `git`         git 操作封装
//! - `web_fetch`   抓网页
//! - `web_search`  调 MiMo 原生 Web Search
//! - `task`        派子 agent
//! - `todo`        任务清单

use serde::{Deserialize, Serialize};

#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn input_schema(&self) -> serde_json::Value;

    async fn run(&self, args: serde_json::Value) -> anyhow::Result<ToolResult>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub output: String,
    pub is_error: bool,
}
