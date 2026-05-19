//! Loads skills from `~/.mimo/skills/` and the workspace's `./.claude/skills/`.

use anyhow::Result;
use mimo_tui_skills::SkillRegistry;
use std::path::Path;
use tracing::info;

use crate::paths;

pub fn load_default_skills(workspace: &Path) -> Result<SkillRegistry> {
    let mut reg = SkillRegistry::new();

    // Global skills.
    if let Ok(global) = paths::skills_dir() {
        let n = reg.load_dir(&global)?;
        if n > 0 {
            info!(dir = %global.display(), count = n, "loaded global skills");
        }
    }

    // Workspace-local skills (Claude Code compatible path).
    let local = workspace.join(".claude").join("skills");
    if local.exists() {
        let n = reg.load_dir(&local)?;
        if n > 0 {
            info!(dir = %local.display(), count = n, "loaded workspace skills");
        }
    }

    // Also accept `.mimo/skills/` per workspace.
    let local2 = workspace.join(".mimo").join("skills");
    if local2.exists() {
        let n = reg.load_dir(&local2)?;
        if n > 0 {
            info!(dir = %local2.display(), count = n, "loaded workspace .mimo skills");
        }
    }

    Ok(reg)
}
