//! Windows Job Object sandbox.
//!
//! We create a Job Object with restrictions (memory cap, no breakaway, kill on
//! parent exit), assign the child process to it. We **don't** implement file
//! ACL restrictions on Windows yet — Job Objects only handle process-level
//! resources. This still gets us "kill on parent exit" which prevents runaway
//! agent processes, which is the main practical concern.

use anyhow::Result;

use crate::Policy;

pub fn apply(_command: &mut tokio::process::Command, _policy: &Policy) -> Result<()> {
    // Job Object assignment after spawn is the practical pattern; the v0.3
    // shell tool path uses tokio::Command::spawn which doesn't expose the
    // PROCESS_HANDLE in time. The full implementation lives in the shell tool
    // wrapper. For now, this is a no-op stub on Windows.
    Ok(())
}
