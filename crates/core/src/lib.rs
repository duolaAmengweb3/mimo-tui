//! 核心运行时：agent loop / 配置 / 会话 / 用量
//!
//! 子模块（待实现）：
//! - `agent`   —— Plan/Agent/Auto 三模式的主循环
//! - `config`  —— `~/.mimo/config.toml` + 环境变量解析
//! - `auth`    —— API key 读写 + 加密
//! - `session` —— 会话保存 / 恢复 / 分叉
//! - `usage`   —— 套餐用量统计 + sqlite
//! - `region`  —— CN / SGP / AMS 延迟测试与路由
//! - `fallback`—— 多模型 fallback

pub mod prelude {
    pub use anyhow::{anyhow, bail, Context, Result};
}
