//! Simple session persistence: dump JSON of every turn to ~/.mimo/sessions/.

use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::paths;
use mimo_tui_anthropic_client::Message;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub created_at: chrono::DateTime<Utc>,
    pub workspace: PathBuf,
    pub model: String,
    pub messages: Vec<Message>,
}

impl Session {
    pub fn new(workspace: PathBuf, model: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            workspace,
            model,
            messages: Vec::new(),
        }
    }

    pub fn path(&self) -> Result<PathBuf> {
        Ok(paths::sessions_dir()?.join(format!("{}.json", self.id)))
    }

    pub fn save(&self) -> Result<()> {
        paths::ensure_layout()?;
        let p = self.path()?;
        std::fs::write(&p, serde_json::to_vec_pretty(self)?)?;
        Ok(())
    }

    pub fn load(id: &str) -> Result<Self> {
        let p = paths::sessions_dir()?.join(format!("{}.json", id));
        Self::load_from(&p)
    }

    pub fn load_from(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&raw)?)
    }

    /// List recently modified sessions on disk.
    pub fn list_recent(limit: usize) -> Result<Vec<SessionMeta>> {
        let dir = paths::sessions_dir()?;
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut entries: Vec<_> = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|x| x == "json").unwrap_or(false))
            .collect();
        entries.sort_by_key(|e| {
            e.metadata()
                .and_then(|m| m.modified())
                .ok()
                .unwrap_or(std::time::UNIX_EPOCH)
        });
        entries.reverse();
        entries.truncate(limit);

        let mut metas = Vec::new();
        for e in entries {
            if let Ok(s) = Session::load_from(&e.path()) {
                metas.push(SessionMeta {
                    id: s.id,
                    created_at: s.created_at,
                    model: s.model,
                    workspace: s.workspace,
                    turn_count: s.messages.len(),
                });
            }
        }
        Ok(metas)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMeta {
    pub id: String,
    pub created_at: chrono::DateTime<Utc>,
    pub model: String,
    pub workspace: PathBuf,
    pub turn_count: usize,
}
