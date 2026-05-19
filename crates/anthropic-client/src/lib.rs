//! 轻量 Anthropic Messages API client
//!
//! 用于接 MiMo Token Plan 的 `/anthropic/v1/messages` endpoint。
//! 兼容标准 Anthropic 协议：
//!   - tool_use / tool_result
//!   - streaming SSE
//!   - thinking 字段
//!   - prompt 缓存 usage
//!
//! 待实现模块：
//! - `messages` —— /v1/messages 调用
//! - `stream`   —— SSE 解析
//! - `types`    —— Request / Response / Message / Content / Usage
//! - `retry`    —— 重试 + 退避策略
//! - `error`    —— 错误类型

pub mod prelude {
    pub use anyhow::{anyhow, bail, Context, Result};
}
