//! Filesystem layout for `~/.mimo/`.

use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn home_dir() -> Result<PathBuf> {
    directories::BaseDirs::new()
        .map(|d| d.home_dir().to_path_buf())
        .context("could not determine home directory")
}

pub fn mimo_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join(".mimo"))
}

pub fn auth_file() -> Result<PathBuf> {
    Ok(mimo_dir()?.join("auth.json"))
}

pub fn config_file() -> Result<PathBuf> {
    Ok(mimo_dir()?.join("config.toml"))
}

pub fn sessions_dir() -> Result<PathBuf> {
    Ok(mimo_dir()?.join("sessions"))
}

pub fn logs_dir() -> Result<PathBuf> {
    Ok(mimo_dir()?.join("logs"))
}

pub fn usage_db() -> Result<PathBuf> {
    Ok(mimo_dir()?.join("usage.db"))
}

pub fn skills_dir() -> Result<PathBuf> {
    Ok(mimo_dir()?.join("skills"))
}

pub fn mcp_dir() -> Result<PathBuf> {
    Ok(mimo_dir()?.join("mcp"))
}

/// Ensure all standard directories exist.
pub fn ensure_layout() -> Result<()> {
    for dir in [mimo_dir()?, sessions_dir()?, logs_dir()?, skills_dir()?, mcp_dir()?] {
        std::fs::create_dir_all(&dir).with_context(|| format!("mkdir {}", dir.display()))?;
    }
    Ok(())
}
