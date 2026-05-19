//! User configuration in `~/.mimo/config.toml`.

use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::paths;
use crate::region::RegionConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub region: RegionConfig,

    #[serde(default = "default_model")]
    pub model: String,

    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,

    #[serde(default)]
    pub mode: AgentModeConfig,

    #[serde(default = "default_max_iterations")]
    pub max_iterations: u32,

    #[serde(default)]
    pub language: Language,

    /// Optional fallback (e.g. DeepSeek / Qwen) endpoint, OpenAI-compatible.
    #[serde(default)]
    pub fallback: Option<FallbackConfig>,
}

fn default_model() -> String {
    "mimo-v2.5-pro".to_string()
}

fn default_max_tokens() -> u32 {
    4096
}

fn default_max_iterations() -> u32 {
    32
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AgentModeConfig {
    Plan,
    #[default]
    Agent,
    Auto,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[default]
    Zh,
    En,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackConfig {
    pub base_url: String,
    pub api_key_env: String,
    pub model: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            region: RegionConfig::default(),
            model: default_model(),
            max_tokens: default_max_tokens(),
            mode: AgentModeConfig::default(),
            max_iterations: default_max_iterations(),
            language: Language::default(),
            fallback: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        Self::load_from(&paths::config_file()?)
    }

    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
        let cfg: Config = toml::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
        Ok(cfg)
    }

    pub fn save(&self) -> Result<()> {
        paths::ensure_layout()?;
        self.save_to(&paths::config_file()?)
    }

    pub fn save_to(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let serialized = toml::to_string_pretty(self)?;
        std::fs::write(path, serialized)?;
        Ok(())
    }

    /// Apply environment-variable overrides on top of file values.
    pub fn apply_env(&mut self) {
        if let Ok(v) = std::env::var("MIMO_REGION") {
            match v.to_lowercase().as_str() {
                "cn" => self.region = RegionConfig::Cn,
                "sgp" => self.region = RegionConfig::Sgp,
                "ams" => self.region = RegionConfig::Ams,
                _ => {}
            }
        }
        if let Ok(v) = std::env::var("MIMO_MODEL") {
            if !v.is_empty() {
                self.model = v;
            }
        }
        if let Ok(v) = std::env::var("MIMO_MODE") {
            match v.to_lowercase().as_str() {
                "plan" => self.mode = AgentModeConfig::Plan,
                "agent" => self.mode = AgentModeConfig::Agent,
                "auto" => self.mode = AgentModeConfig::Auto,
                _ => {}
            }
        }
    }
}
