//! API key storage (`~/.mimo/auth.json`).

use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::paths;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auth {
    pub api_key: String,
}

impl Auth {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
        }
    }

    /// Load from default location.
    pub fn load() -> Result<Option<Self>> {
        Self::load_from(&paths::auth_file()?)
    }

    pub fn load_from(path: &Path) -> Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }
        let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        let auth: Auth = serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
        Ok(Some(auth))
    }

    /// Save with mode 0600 (owner-only).
    pub fn save(&self) -> Result<()> {
        paths::ensure_layout()?;
        self.save_to(&paths::auth_file()?)
    }

    pub fn save_to(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized = serde_json::to_string_pretty(self)?;

        // Write with 0600 perms on Unix.
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            let mut f = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .mode(0o600)
                .open(path)?;
            f.write_all(serialized.as_bytes())?;
        }
        #[cfg(not(unix))]
        {
            fs::write(path, serialized)?;
        }
        Ok(())
    }

    /// Convenience: pick up `MIMO_API_KEY` from env, otherwise load from file.
    pub fn resolve() -> Result<Option<Self>> {
        if let Ok(k) = std::env::var("MIMO_API_KEY") {
            if !k.is_empty() {
                return Ok(Some(Auth::new(k)));
            }
        }
        Self::load()
    }
}
