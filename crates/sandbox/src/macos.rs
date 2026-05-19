//! macOS Seatbelt sandbox.
//!
//! We use `sandbox-exec(1)` (built into macOS) with an inline `.sb` profile.
//! The profile allows everything **except** writing outside the workspace.

use anyhow::Result;

use crate::Policy;

pub fn apply(command: &mut tokio::process::Command, policy: &Policy) -> Result<()> {
    let workspace = policy.workspace.display();
    let profile = format!(
        r#"(version 1)
(allow default)
(deny file-write*)
(allow file-write*
    (subpath "{workspace}")
    (subpath "/tmp")
    (subpath "/private/tmp")
    (subpath "/var/folders")
    (literal "/dev/null")
    (literal "/dev/dtracehelper")
    (regex #"^/private/var/folders/.+"))
(allow file-read*)
"#
    );

    let original_program = command.as_std().get_program().to_owned();
    let original_args: Vec<std::ffi::OsString> =
        command.as_std().get_args().map(|s| s.to_owned()).collect();

    // Rewrap: sandbox-exec -p <profile> -- <original> <args...>
    *command = tokio::process::Command::new("sandbox-exec");
    command.arg("-p").arg(profile).arg("--").arg(original_program);
    for a in original_args {
        command.arg(a);
    }
    Ok(())
}
