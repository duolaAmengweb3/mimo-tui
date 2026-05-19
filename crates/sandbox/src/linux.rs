//! Linux Landlock filesystem sandbox.
//!
//! Landlock is the Linux kernel's native sandbox for filesystem access. We
//! apply the ruleset via `pre_exec` so the child process is locked down
//! before its `execve`.

use std::os::unix::process::CommandExt;

use anyhow::{Context, Result};
use landlock::{
    Access, AccessFs, PathBeneath, PathFd, Ruleset, RulesetAttr, RulesetCreatedAttr, ABI,
};

use crate::Policy;

pub fn apply(command: &mut tokio::process::Command, policy: &Policy) -> Result<()> {
    let workspace = policy.workspace.clone();
    let read_only_paths = policy.read_only_paths.clone();

    // Safety: we only call kernel APIs in the pre_exec closure.
    unsafe {
        command.as_std_mut().pre_exec(move || {
            let abi = ABI::V2;
            let read_only = AccessFs::from_read(abi);
            let read_write = AccessFs::from_all(abi);

            let ruleset = Ruleset::default()
                .handle_access(read_write)
                .map_err(io_err)?
                .create()
                .map_err(io_err)?;

            // Read+write on workspace
            let ruleset = if let Ok(fd) = PathFd::new(&workspace) {
                ruleset
                    .add_rule(PathBeneath::new(fd, read_write))
                    .map_err(io_err)?
            } else {
                ruleset
            };

            // Read-only on `/` so commands like `which`, library loads, etc work.
            let mut ruleset = if let Ok(fd) = PathFd::new("/") {
                ruleset
                    .add_rule(PathBeneath::new(fd, read_only))
                    .map_err(io_err)?
            } else {
                ruleset
            };

            for path in &read_only_paths {
                if let Ok(fd) = PathFd::new(path) {
                    ruleset = ruleset
                        .add_rule(PathBeneath::new(fd, read_only))
                        .map_err(io_err)?;
                }
            }

            ruleset.restrict_self().map_err(io_err)?;
            Ok(())
        });
    }
    Ok(())
}

fn io_err<E: std::fmt::Display>(e: E) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::Other, format!("landlock: {}", e))
}

#[allow(dead_code)]
fn must_use_context() -> Result<()> {
    Err(anyhow::anyhow!("internal").context("landlock"))
}
