//! Generic fallback for unsupported OSes (BSDs, etc.) — no-op.

use anyhow::Result;

use crate::Policy;

pub fn apply(_command: &mut tokio::process::Command, _policy: &Policy) -> Result<()> {
    Ok(())
}
