//! mimo-tui CLI entry point.

mod ui;
mod wizard;

use std::sync::Arc;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use mimo_tui_anthropic_client::Client;
use mimo_tui_core::{
    agent::{approval_mode, Agent},
    auth::Auth,
    config::Config,
    load_default_skills, paths,
    session::Session,
    McpHub,
};
use mimo_tui_tools::{ToolContext, ToolRegistry};
use tokio::sync::mpsc;

#[derive(Parser)]
#[command(
    name = "mimo",
    version,
    about = "MiMo 模型的原创开源终端 AI 编程 agent"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Print debug logs.
    #[arg(long, env = "MIMO_DEBUG")]
    debug: bool,

    /// One-shot mode: run `prompt` and exit (no REPL).
    #[arg(short = 'p', long = "print")]
    one_shot: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// First-time setup wizard (API key + region + default model).
    Init,
    /// Auth management.
    Auth {
        #[command(subcommand)]
        cmd: AuthCmd,
    },
    /// Show usage stats from the local DB.
    Usage,
}

#[derive(Subcommand)]
enum AuthCmd {
    /// Save / overwrite the API key.
    Login,
    /// Remove the saved API key.
    Logout,
    /// Show current auth status.
    Status,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let level = if cli.debug { "debug" } else { "warn" };
    tracing_subscriber::fmt()
        .with_env_filter(level)
        .with_target(false)
        .with_writer(std::io::stderr)
        .init();

    paths::ensure_layout().ok();

    match cli.command {
        Some(Commands::Init) => return wizard::run().await,
        Some(Commands::Auth { cmd }) => return handle_auth(cmd).await,
        Some(Commands::Usage) => return show_usage(),
        None => {}
    }

    // No subcommand → run the agent.
    run_agent(cli.one_shot).await
}

async fn handle_auth(cmd: AuthCmd) -> Result<()> {
    match cmd {
        AuthCmd::Login => wizard::run().await,
        AuthCmd::Logout => {
            let path = paths::auth_file()?;
            if path.exists() {
                std::fs::remove_file(&path)?;
                println!("✓ removed {}", path.display());
            } else {
                println!("(no auth file)");
            }
            Ok(())
        }
        AuthCmd::Status => {
            match Auth::resolve()? {
                Some(_) => {
                    let from_env = std::env::var("MIMO_API_KEY").is_ok();
                    if from_env {
                        println!("✓ authenticated (key from MIMO_API_KEY env)");
                    } else {
                        println!(
                            "✓ authenticated (key from {})",
                            paths::auth_file()?.display()
                        );
                    }
                }
                None => println!("✗ not authenticated. run `mimo init`"),
            }
            Ok(())
        }
    }
}

fn show_usage() -> Result<()> {
    let db = mimo_tui_core::usage::UsageDb::open()?;
    let today = db.totals_today()?;
    println!("==== mimo-tui · usage today ({}) ====", today.date);
    println!("  API calls:       {}", today.call_count);
    println!("  Input tokens:    {}", today.input_tokens);
    println!("  Output tokens:   {}", today.output_tokens);
    println!("  Cache hit:       {} tokens", today.cache_read_tokens);
    Ok(())
}

async fn run_agent(one_shot: Option<String>) -> Result<()> {
    // 1. Auth resolution: env → file → wizard.
    let _auth = match Auth::resolve()? {
        Some(a) => a,
        None => {
            println!("(no API key configured yet — running first-run wizard)\n");
            wizard::run().await?;
            Auth::resolve()?.context("auth still missing after wizard")?
        }
    };

    if let Some(prompt) = one_shot {
        return run_one_shot_path(prompt).await;
    }

    // Interactive: full-screen ratatui TUI.
    mimo_tui_tui::run().await
}

/// One-shot path: rebuilds a minimal agent and runs a single turn, printing
/// streamed output to stdout (no TUI). Useful for scripting / CI.
async fn run_one_shot_path(prompt: String) -> Result<()> {
    let auth = Auth::resolve()?.context("auth missing")?;
    let mut config = Config::load()?;
    config.apply_env();
    let workspace = std::env::current_dir()?;
    let client = Client::new(auth.api_key, config.region.to_client_region());

    let mut registry = ToolRegistry::with_defaults();
    let _mcp_hub = McpHub::init(&mut registry).await.ok();
    let tools = Arc::new(registry);
    let skills = Arc::new(load_default_skills(&workspace).unwrap_or_default());
    let ctx = ToolContext::new(workspace.clone()).with_mode(approval_mode(config.mode));
    let session = Session::new(workspace, config.model.clone());
    let mut agent = Agent::new(client, config, tools, ctx, session).with_skills(skills);
    run_one_shot(&mut agent, &prompt).await
}

async fn run_one_shot(agent: &mut Agent, prompt: &str) -> Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let agent_fut = agent.run_turn(prompt, tx);
    let render_task = tokio::spawn(async move {
        while let Some(ev) = rx.recv().await {
            ui::render_event(ev);
        }
    });
    let _ = agent_fut.await?;
    let _ = render_task.await;
    Ok(())
}
