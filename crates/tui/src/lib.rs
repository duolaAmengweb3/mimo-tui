//! TUI 渲染层
//!
//! 主要组件（待实现）：
//! - `app`        主应用状态
//! - `ui::main`   主对话界面
//! - `ui::input`  输入框
//! - `ui::status` 状态栏（模型/模式/集群/用量）
//! - `ui::sidebar` 侧边栏
//! - `ui::wizard` 首次启动配置向导
//! - `ui::usage`  /usage 详细看板
//! - `ui::welcome`欢迎屏（ASCII logo）
//! - `event`      键盘 + 鼠标 + tick 事件循环

use anyhow::Result;

pub async fn run() -> Result<()> {
    // TODO: 启动 TUI 主循环
    println!("∞ mimo-tui · TUI 运行时（待实现）");
    println!("详见 PRD.md 第 6 节 + 上线清单/04-Rust主项目.md");
    Ok(())
}
