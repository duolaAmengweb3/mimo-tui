//! Core runtime: config / auth / agent loop / sessions / usage / mcp / skills.

pub mod agent;
pub mod auth;
pub mod config;
pub mod mcp_bridge;
pub mod paths;
pub mod region;
pub mod session;
pub mod skills_bridge;
pub mod usage;

pub use agent::{Agent, AgentEvent, AgentReply};
pub use auth::Auth;
pub use config::Config;
pub use mcp_bridge::McpHub;
pub use skills_bridge::load_default_skills;

pub mod prelude {
    pub use anyhow::{anyhow, bail, Context, Result};
}
