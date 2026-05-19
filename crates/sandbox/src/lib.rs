//! Cross-platform sandbox for the `shell` tool.
//!
//! Each platform wraps `tokio::process::Command` and applies an OS-native
//! restriction:
//!
//! | OS      | Mechanism                                        |
//! |---------|--------------------------------------------------|
//! | macOS   | `sandbox-exec` (Seatbelt) with a generated `.sb` |
//! | Linux   | Landlock filesystem ruleset                      |
//! | Windows | Job Object (restricts process tree resources)    |
//!
//! Default policy:
//! - Read/write only inside `workspace`
//! - Read-only access to the rest of the filesystem
//! - Network: allowed (we can't reliably block it everywhere)
//! - exec: shell + listed allowlist

use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Sandbox policy a tool wants applied.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub workspace: PathBuf,
    #[serde(default)]
    pub read_only_paths: Vec<PathBuf>,
    /// If true, run the command without any sandbox at all (Auto mode override).
    #[serde(default)]
    pub disabled: bool,
}

impl Policy {
    pub fn workspace_only(workspace: PathBuf) -> Self {
        Self {
            workspace,
            read_only_paths: Vec::new(),
            disabled: false,
        }
    }

    pub fn disabled() -> Self {
        Self {
            workspace: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            read_only_paths: Vec::new(),
            disabled: true,
        }
    }
}

pub mod sb {
    //! Cross-platform helpers re-exported from each backend.
    pub use super::Policy;
}

#[cfg(target_os = "macos")]
mod backend {
    pub use super::macos::*;
}
#[cfg(target_os = "linux")]
mod backend {
    pub use super::linux::*;
}
#[cfg(windows)]
mod backend {
    pub use super::windows_impl::*;
}
#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
mod backend {
    pub use super::generic::*;
}

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(windows)]
mod windows_impl;
#[cfg(not(any(target_os = "macos", target_os = "linux", windows)))]
mod generic;

/// Wrap a `tokio::process::Command` with the platform sandbox.
/// Mutates the command in place; safe to call repeatedly.
pub fn apply(command: &mut tokio::process::Command, policy: &Policy) -> Result<()> {
    if policy.disabled {
        return Ok(());
    }
    backend::apply(command, policy)
}
