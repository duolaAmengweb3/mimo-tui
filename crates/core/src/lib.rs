//! Core runtime: config / auth / agent loop / sessions / usage.

pub mod agent;
pub mod auth;
pub mod config;
pub mod paths;
pub mod region;
pub mod session;
pub mod usage;

pub use agent::{Agent, AgentEvent, AgentReply};
pub use auth::Auth;
pub use config::Config;

pub mod prelude {
    pub use anyhow::{anyhow, bail, Context, Result};
}
