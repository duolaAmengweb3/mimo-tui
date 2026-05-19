//! 三平台沙箱
//!
//! - macOS：Seatbelt（sandbox-exec）
//! - Linux：Landlock + seccomp
//! - Windows：Job Objects + Restricted Token
//!
//! 默认策略：
//! - 工作目录可读写
//! - 工作目录外只读
//! - shell 命令走白名单 + 用户审批

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(windows)]
pub mod windows;

use anyhow::Result;

pub trait Sandbox {
    fn allow_read(&mut self, path: &std::path::Path) -> Result<()>;
    fn allow_write(&mut self, path: &std::path::Path) -> Result<()>;
    fn enter(&self) -> Result<()>;
}
