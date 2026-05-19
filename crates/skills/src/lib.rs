//! Skills 系统（完全兼容 Anthropic Skills 格式）
//!
//! 用户的 Claude Code skill 目录（`.claude/skills/<name>.md`）可以**直接拷贝**到
//! mimo-tui 的 skill 目录使用，零迁移成本。
//!
//! 待实现：
//! - `loader`    解析 frontmatter + markdown body
//! - `registry`  全局 skill 注册
//! - `dispatch`  根据触发条件激活
//! - `install`   `/skill install <github-repo>`
//!
//! 内置 skills：
//! - mimo-cache-optimizer
//! - china-dev-essentials
//! - bug-hunter
//! - refactor-pro
//! - doc-writer
